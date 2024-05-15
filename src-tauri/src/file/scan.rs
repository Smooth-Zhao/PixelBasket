use std::path::{Path, PathBuf};
use std::time::Instant;
use tauri::async_runtime::TokioRuntime;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::config::get_db_path;
use crate::db::entity::basket::{Basket, BasketData};
use crate::db::entity::folder::Folder;
use crate::db::entity::metadata::Metadata;
use crate::db::entity::task::{Task, TaskStatus};
use crate::db::sqlite::Session;
use crate::util::error::ErrorHandle;
use crate::util::snowflake::id_str;
use crate::{debug, info, Result};

pub struct Context {
    pub runtime: TokioRuntime,
}

pub trait Scanner {
    fn is_support(&self, suffix: &str) -> bool;
    fn scan(&self, task: &Task, context: &Context) -> TaskStatus;
}

pub struct ScanMsg {
    pub r#type: String,
    pub data: String,
}

impl ScanMsg {
    pub fn new(r#type: String, data: String) -> Self {
        Self { r#type, data }
    }
}

pub struct ScanJob {
    id: String,
    scanners: Vec<Box<dyn Scanner + Send>>,
    tx: Sender<ScanMsg>,
    pub file_list: Vec<PathBuf>,
    pub file_count: usize,
    pub task_count: usize,
    pub scan_count: usize,
    pub parent_folder_id: i64,
    pub folder_list: Vec<Folder>,
    pub basket_name: String,
    pub directories: Vec<String>,
    pub cpu_nums: usize,
}

impl ScanJob {
    pub fn new(tx: Sender<ScanMsg>) -> Self {
        Self {
            id: id_str(),
            scanners: Vec::new(),
            tx,
            file_list: Vec::new(),
            file_count: 0,
            task_count: 0,
            scan_count: 0,
            parent_folder_id: 0,
            folder_list: Vec::new(),
            basket_name: String::new(),
            directories: Vec::new(),
            cpu_nums: num_cpus::get() / 2,
        }
    }

    pub fn add_scanners(&mut self, scanners: Vec<Box<dyn Scanner + Send>>) {
        self.scanners = scanners;
    }

    pub async fn run(&mut self, directories: Vec<String>) {
        // 文件读取
        self.load_dir(directories).await;

        let mut session = Session::new(get_db_path());
        session.connect().await;

        // 保存文件信息
        self.save_folder(&session).await;
        self.save_basket(&session).await;
        // 扫描任务处理
        self.load_task(&session).await;
        self.run_scanner(&session).await;
    }

    pub async fn run_task(&mut self) {
        let mut session = Session::new(get_db_path());
        session.connect().await;
        // 扫描任务处理
        self.run_scanner(&session).await;
    }

    pub async fn load_dir(&mut self, directories: Vec<String>) {
        let start = Instant::now();
        directories.iter().map(|v| Path::new(v)).for_each(|v| {
            info!("<scan:{}> 路径：{:?}", self.id, v.as_os_str());
            self.load_file_list(v).unwrap();
            self.file_count = self.file_list.len();
        });
        self.tx
            .send(ScanMsg::new(
                "file".to_string(),
                self.file_count.to_string(),
            ))
            .await
            .print_error();
        info!(
            "<scan:{}> 加载{}个文件,代码运行时间为{:?}秒",
            self.id,
            self.file_count,
            (Instant::now() - start).as_secs()
        );
    }

    fn load_file_list(&mut self, path: &Path) -> Result<()> {
        if path.is_dir() && !is_hidden(path) {
            let folder = Folder::new(&path, self.parent_folder_id);
            let folder_id = folder.id;
            self.folder_list.push(folder);
            let read = path.read_dir()?;
            for entry in read.flatten() {
                self.parent_folder_id = folder_id;
                self.load_file_list(entry.path().as_path())?
            }
        } else if path.is_file() && self.is_support(path) {
            self.file_list.push(path.to_path_buf());
        }
        Ok(())
    }

    fn is_support(&self, path: &Path) -> bool {
        if let Some(suffix) = get_file_suffix(path) {
            let string = suffix.to_lowercase();
            return self.scanners.iter().any(|v| v.is_support(&string));
        }
        false
    }

    pub async fn load_task(&mut self, session: &Session) {
        let start = Instant::now();

        for path in self.file_list.iter() {
            let metadata = Metadata::load(path);
            metadata.save_task_to_db(session).await;
            self.task_count += 1;
        }

        self.tx
            .send(ScanMsg::new(
                "task".to_string(),
                self.task_count.to_string(),
            ))
            .await
            .print_error();
        info!(
            "<scan:{}> 创建{}个任务,代码运行时间为{:?}秒",
            self.id,
            self.task_count,
            (Instant::now() - start).as_secs()
        );
    }

    pub async fn run_scanner(&mut self, session: &Session) {
        let start = Instant::now();

        if let Some(task_list) = session
            .select_as::<Task>("SELECT * FROM task WHERE status = 0")
            .await
            .print_error()
        {
            if let Some(runtime) = tokio::runtime::Builder::new_multi_thread()
                .max_blocking_threads(self.cpu_nums)
                .enable_all()
                .build()
                .print_error()
            {
                let context = Context { runtime };
                let mut handles = Vec::new();
                for task in task_list.iter() {
                    for scanner in self.scanners.iter() {
                        handles.push(scanner.scan(task, &context));
                    }
                }
                for status in handles {
                    let id = status.id;
                    let is_success = status.success().await;
                    if is_success {
                        self.scan_count += 1;
                        let sql = format!("DELETE FROM task WHERE id = {}", id);
                        session.execute(&sql).await.print_error();
                        debug!("<scan:{}> 执行任务<id:{}>完成", self.id, id);
                    }
                }

                context.runtime.shutdown_background();

                self.tx
                    .send(ScanMsg::new(
                        "done".to_string(),
                        self.scan_count.to_string(),
                    ))
                    .await
                    .print_error();
                info!(
                    "<scan:{}> 执行{}个任务,代码运行时间为{:?}秒",
                    self.id,
                    self.scan_count,
                    (Instant::now() - start).as_secs()
                );
            }
        }
    }

    pub fn run_async(mut self, basket: BasketData) {
        self.basket_name = basket.name;
        self.directories = basket.directories;
        tokio::spawn(async move { self.run(self.directories.clone()).await });
    }

    pub fn run_task_async(mut self) {
        tokio::spawn(async move { self.run_task().await });
    }

    pub fn monitor_async(&self, mut rx: Receiver<ScanMsg>) {
        let id = self.id.clone();
        tokio::spawn(async move {
            loop {
                if let Some(msg) = rx.recv().await {
                    debug!("{} -> {}", msg.r#type, msg.data);
                    if msg.r#type == "done" {
                        break;
                    }
                }
            }
            info!("<scan:{}> 扫描结束", id);
        });
    }

    pub async fn save_folder(&mut self, session: &Session) {
        let start = Instant::now();
        for folder in self.folder_list.iter_mut() {
            if !folder.exist(&session).await {
                folder.save(&session).await;
            } else {
                if let Some(parent) = get_path_parent(&folder.path) {
                    if let Some(parent) = Folder::get_by_path(&session, parent).await {
                        if let Some(current) =
                            Folder::get_by_path(&session, folder.path.clone()).await
                        {
                            if current.pid == 0 {
                                folder.id = current.id;
                                folder.pid = parent.id;
                                folder.update(&session).await;
                            }
                        }
                    }
                }
            }
        }
        info!(
            "<scan:{}> 保存{}个文件夹,代码运行时间为{:?}秒",
            self.id,
            self.folder_list.len(),
            (Instant::now() - start).as_secs()
        );
    }

    pub async fn save_basket(&mut self, session: &Session) {
        let start = Instant::now();
        let basket = Basket::new(self.basket_name.clone());
        if !basket.exist(&session).await {
            basket.save(&session).await;
        }
        basket.save_folder(&self.directories, &session).await;
        info!(
            "<scan:{}> 保存篮子信息,代码运行时间为{:?}秒",
            self.id,
            (Instant::now() - start).as_secs()
        );
    }
}

fn is_hidden(path: &Path) -> bool {
    if let Some(file_name) = path.file_name() {
        if let Some(file_name) = file_name.to_str() {
            if file_name.starts_with('.') {
                return true;
            }
        }
    }
    false
}

fn get_file_suffix(path: &Path) -> Option<&str> {
    path.extension()?.to_str()
}

fn get_path_parent(path: &str) -> Option<String> {
    let path = Path::new(path);
    let parent = path.parent();
    if let Some(path) = parent {
        if let Some(path) = path.to_str() {
            return Some(path.to_string());
        }
    }
    None
}
