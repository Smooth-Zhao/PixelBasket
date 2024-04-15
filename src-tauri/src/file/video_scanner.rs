use std::path::Path;

use tokio::sync::mpsc::Sender;

use crate::file::metadata::Metadata;
use crate::file::scan::Scanner;
use crate::Result;
use crate::util::error::ErrorHandle;

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

    fn scan(&self, path: &Path, tx: Sender<String>) -> Result<()> {
        let mut metadata = Metadata::load(path);
        let path = path.to_path_buf();
        if self.is_support(metadata.file_suffix.as_str()) {
            tokio::spawn(async move {
                if metadata.analyze_metadata(&path).is_ok() {
                    metadata.save_to_db().await;
                    tx.send(metadata.file_path).await.print_error();
                };
            });
        }
        Ok(())
    }
}
