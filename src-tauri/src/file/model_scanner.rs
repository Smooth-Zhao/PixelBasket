use std::path::Path;
use crate::file::scan::Scanner;

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
            _ => false
        }
    }

    fn scan(&self, _path: &Path) -> crate::Result<bool> {
        Ok(false)
    }
}
