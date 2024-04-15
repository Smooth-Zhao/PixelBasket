use std::path::{Path, PathBuf};
use std::time::Instant;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::{debug, info, Result};

pub trait Scanner {
    fn is_support(&self, suffix: &str) -> bool;
    fn scan(&self, path: &Path, tx: Sender<String>) -> Result<()>;
}

pub struct ScanJob {
    scanners: Vec<Box<dyn Scanner + Send>>,
    tx: Sender<String>,
    pub file_list: Vec<PathBuf>,
    pub file_count: usize,
    pub scan_count: usize,
}

impl ScanJob {
    pub fn new(tx: Sender<String>) -> Self {
        Self {
            tx,
            scanners: Vec::new(),
            file_list: Vec::new(),
            file_count: 0,
            scan_count: 0,
        }
    }

    pub fn add_scanners(&mut self, scanners: Vec<Box<dyn Scanner + Send>>) {
        self.scanners = scanners;
    }

    pub fn run(&mut self, directories: Vec<String>) {
        self.load_dir(directories);
        self.run_scanner();
    }

    pub fn load_dir(&mut self, directories: Vec<String>) {
        let start = Instant::now();
        directories.iter().map(|v| Path::new(v)).for_each(|v| {
            info!("路径：{:?}", v.as_os_str());
            self.load_file_list(v).unwrap();
            self.file_count = self.file_list.len();
        });
        info!(
            "加载{}个文件,代码运行时间为{:?}秒",
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

    pub fn run_scanner(&mut self) {
        let start = Instant::now();

        for path in self.file_list.iter() {
            for scanner in self.scanners.iter() {
                let _ = scanner.scan(path, self.tx.clone());
            }
        }

        info!(
            "扫描{}个文件,代码运行时间为{:?}秒",
            self.file_count,
            (Instant::now() - start).as_secs()
        );
    }

    pub fn run_async(mut self, directories: Vec<String>) {
        tokio::spawn(async move { self.run(directories) });
    }

    pub fn monitor_async(&self, mut rx: Receiver<String>) {
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                debug!("{}", msg);
            }
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
