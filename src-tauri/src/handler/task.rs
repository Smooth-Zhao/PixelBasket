use tokio::sync::mpsc::channel;

use crate::scanner::image_scanner::ImageScanner;
use crate::scanner::model_scanner::ModelScanner;
use crate::scanner::psd_scanner::PsdScanner;
use crate::scanner::raw_scanner::RawScanner;
use crate::scanner::scan::{ScanJob, ScanMsg};
use crate::scanner::video_scanner::VideoScanner;

#[tauri::command]
pub fn run_task() -> &'static str {
    let (tx, rx) = channel::<ScanMsg>(16);
    let mut scan = ScanJob::new(tx);
    scan.add_scanners(vec![
        ImageScanner::wrap(),
        ModelScanner::wrap(),
        VideoScanner::wrap(),
        RawScanner::wrap(),
        PsdScanner::wrap(),
    ]);
    scan.monitor_async(rx);
    scan.run_task_async();
    "OK"
}
