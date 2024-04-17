use std::path::Path;

use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

use crate::file::metadata::Metadata;
use crate::file::scan::{ScanMsg, Scanner};
use crate::util::error::ErrorHandle;

pub struct ModelScanner {}

impl ModelScanner {
    pub fn wrap() -> Box<Self> {
        Box::new(ModelScanner {})
    }
}

impl Scanner for ModelScanner {
    fn is_support(&self, suffix: &str) -> bool {
        match suffix {
            "obj" | "fbx" => true,
            _ => false,
        }
    }

    fn scan(&self, path: &Path, tx: Sender<ScanMsg>) -> Option<JoinHandle<()>> {
        let mut metadata = Metadata::load(path);
        let path = path.to_path_buf();
        if self.is_support(metadata.file_suffix.as_str()) {
            return Some(tokio::spawn(async move {
                if metadata.analyze_metadata(&path).is_ok() {
                    metadata.save_to_db().await;
                    tx.send(ScanMsg::new("path".to_string(), metadata.full_path))
                        .await
                        .print_error();
                };
            }));
        }
        None
    }
}
