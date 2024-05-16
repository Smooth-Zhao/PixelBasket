use std::path::Path;

use crate::db::entity::metadata::Metadata;
use crate::db::entity::task::{Task, TaskStatus};
use crate::file::scan::{Context, Scanner};

pub struct ModelScanner {}

impl ModelScanner {
    pub fn wrap() -> Box<Self> {
        Box::new(ModelScanner {})
    }
}

impl Scanner for ModelScanner {
    fn is_support(&self, suffix: &str) -> bool {
        match suffix.to_lowercase().as_str() {
            "obj" | "fbx" => true,
            _ => false,
        }
    }

    fn scan(&self, task: &Task, context: &Context) -> TaskStatus {
        let mut status = TaskStatus::new(task.id);
        if self.is_support(task.file_suffix.as_str()) {
            let path = task.file_path.clone();
            let runtime = context.runtime.handle().clone();
            status.handle(runtime.clone().spawn_blocking(move || {
                let path = Path::new(path.as_str());
                let mut metadata = Metadata::load(path);
                if metadata.analyze_metadata(path).is_ok() {
                    // 使用阻塞线程防止数据丢失！
                    runtime.block_on(async move {
                        metadata.save_to_db().await;
                    });
                    return true;
                };
                false
            }));
        }
        status
    }
}
