use std::path::Path;

use image::{DynamicImage, EncodableLayout, ImageBuffer, Rgba};
use psd::Psd;

use crate::db::entity::metadata::Metadata;
use crate::db::entity::task::{Task, TaskStatus};
use crate::file::image_scanner::image_to_base64;
use crate::file::scan::{Context, Scanner};
use crate::Result;

pub struct PsdScanner {}

impl PsdScanner {
    pub fn wrap() -> Box<Self> {
        Box::new(PsdScanner {})
    }
}

impl Scanner for PsdScanner {
    fn is_support(&self, suffix: &str) -> bool {
        match suffix {
            "psd" => true,
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
                    if analyze_psd_metadata(path, &mut metadata).is_ok() {
                        runtime.block_on(async move {
                            metadata.save_to_db().await;
                        });
                        return true;
                    };
                };
                false
            }));
        }
        status
    }
}

/// 解析图片元数据
fn analyze_psd_metadata(path: &Path, metadata: &mut Metadata) -> Result<()> {
    let psd = Psd::from_bytes(std::fs::read(path).unwrap().as_bytes()).unwrap();
    metadata.image_width = psd.width();
    metadata.image_height = psd.height();
    println!("{}*{}", metadata.image_width, metadata.image_height);

    let final_image = psd.rgba();
    // 创建 RgbaImage 对象
    let mut image_buffer = ImageBuffer::new(metadata.image_width, metadata.image_height);
    // 将 RGBA 数据填充到图像中
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let index = (y as usize * metadata.image_width as usize + x as usize) * 4;
        let rgba = Rgba([
            final_image[index],
            final_image[index + 1],
            final_image[index + 2],
            final_image[index + 3],
        ]);
        *pixel = rgba;
    }

    let image = DynamicImage::ImageRgba8(image_buffer);
    let resize_image = if metadata.image_width > 200 {
        crate::file::image_scanner::thumbnail(&image, metadata.image_width, metadata.image_height)
    } else {
        image.to_rgb8()
    };
    if let Some(base64) = image_to_base64(&resize_image) {
        metadata.thumbnail = base64;
    }

    Ok(())
}
