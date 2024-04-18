use std::path::{Path, PathBuf};
use std::time::Instant;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::db::sqlite::Session;
use crate::file::metadata::Metadata;
use crate::file::task::{Task, TaskStatus};
use crate::util::error::ErrorHandle;
use crate::util::snowflake::id_str;
use crate::{debug, info, Result};

pub trait Scanner {
    fn is_support(&self, suffix: &str) -> bool;
    fn scan(&self, task: &Task) -> TaskStatus;
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
        }
    }

    pub fn add_scanners(&mut self, scanners: Vec<Box<dyn Scanner + Send>>) {
        self.scanners = scanners;
    }

    pub async fn run(&mut self, directories: Vec<String>) {
        self.load_dir(directories).await;
        self.load_task().await;
        self.run_scanner().await;
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
            let read = path.read_dir()?;
            for entry in read.flatten() {
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

    pub async fn load_task(&mut self) {
        let start = Instant::now();

        for path in self.file_list.iter() {
            let metadata = Metadata::load(path);
            metadata.save_task_to_db().await;
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

    pub async fn run_scanner(&mut self) {
        let start = Instant::now();

        let mut session = Session::new("./db/main.db");
        session.connect().await;
        if let Some(task_list) = session
            .select_as::<Task>("SELECT * FROM task WHERE status = 0")
            .await
            .print_error()
        {
            let mut handles = Vec::new();
            for task in task_list.iter() {
                for scanner in self.scanners.iter() {
                    handles.push(scanner.scan(task));
                }
            }
            self.task_count = handles.len();
            for status in handles {
                let id = status.id;
                let is_success = status.success().await;
                debug!("<scan:{}> task_id:{},return:{}", self.id, id, is_success);
            }

            self.tx
                .send(ScanMsg::new(
                    "done".to_string(),
                    self.task_count.to_string(),
                ))
                .await
                .print_error();
            info!(
                "<scan:{}> 执行{}个任务,代码运行时间为{:?}秒",
                self.id,
                self.task_count,
                (Instant::now() - start).as_secs()
            );
        }
    }

    pub fn run_async(mut self, directories: Vec<String>) {
        tokio::spawn(async move { self.run(directories).await });
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
