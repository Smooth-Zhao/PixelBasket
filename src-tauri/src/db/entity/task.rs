use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

use crate::util::error::ErrorHandle;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub file_path: String,
    pub file_suffix: String,
    pub status: u8,
}

impl Task {
    pub fn as_status(&self) -> TaskStatus {
        TaskStatus {
            id: self.id,
            success: false,
            handle: None,
        }
    }
}

pub struct TaskStatus {
    pub id: i64,
    success: bool,
    handle: Option<JoinHandle<bool>>,
}

impl TaskStatus {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            success: false,
            handle: None,
        }
    }
    pub fn handle(&mut self, handle: JoinHandle<bool>) {
        self.handle = Some(handle);
    }
    pub fn done(&mut self) {
        self.success = true;
    }
    pub async fn success(self) -> bool {
        match self.handle {
            Some(handle) => {
                handle.await.print_error().is_some_and(|v| v)
            }
            None => self.success,
        }
    }
}
