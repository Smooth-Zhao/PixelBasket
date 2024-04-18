use std::path::Path;

use crate::file::metadata::Metadata;
use crate::file::scan::Scanner;
use crate::file::task::{Task, TaskStatus};

pub struct VideoScanner {}

impl VideoScanner {
    pub fn wrap() -> Box<Self> {
        Box::new(VideoScanner {})
    }
}

impl Scanner for VideoScanner {
    fn is_support(&self, suffix: &str) -> bool {
        match suffix {
            "mp4" | "webm" | "ogg" => true,
            _ => false,
        }
    }

    fn scan(&self, task: &Task) -> TaskStatus {
        let mut status = TaskStatus::new(task.id);
        if self.is_support(task.file_suffix.as_str()) {
            let path = task.file_path.clone();
            status.handle(tokio::spawn(async move {
                let path = Path::new(path.as_str());
                let mut metadata = Metadata::load(path);
                if metadata.analyze_metadata(path).is_ok() {
                    metadata.save_to_db().await;
                    return true;
                };
                false
            }));
        }
        status
    }
}
