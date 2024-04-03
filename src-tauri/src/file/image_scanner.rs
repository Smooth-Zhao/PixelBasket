use crate::file::scan::Scanner;
use chrono::offset::Utc;
use chrono::DateTime;
use image::imageops::FilterType;
use image::{GenericImageView, ImageFormat};
use std::fmt::Display;
use std::path::Path;

pub struct ImageScanner {}

impl ImageScanner {
    pub fn wrap() -> Box<Self> {
        Box::new(ImageScanner {})
    }
}

impl Scanner for ImageScanner {
    fn scan(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            if let Some(file_suffix) = extension.to_str() {
                match file_suffix.to_lowercase().as_str() {
                    "avif" | "bmp" | "dds" | "farbfeld" | "gif" | "hdr" | "ico" | "jpg"
                    | "jpeg" | "exr" | "png" | "pnm" | "qoi" | "tga" | "tiff" | "webp" => {
                        println!("==> {}", path.display());
                        if let Some(file_name) = path.file_name() {
                            println!("文件名：{}", file_name.to_string_lossy());
                        }
                        if let Ok(metadata) = path.metadata() {
                            println!("文件大小：{}", metadata.len());
                            println!("格式：{:?}", file_suffix);
                            if let Ok(time) = metadata.created() {
                                let datetime: DateTime<Utc> = time.into();
                                println!("创建日期：{}", datetime.format("%Y-%m-%d %H:%M:%S"));
                            }
                            if let Ok(time) = metadata.modified() {
                                let datetime: DateTime<Utc> = time.into();
                                println!("修改日期：{}", datetime.format("%Y-%m-%d %H:%M:%S"));
                            }
                        }
                        if let Ok(image) = image::open(path) {
                            // let dimensions = image.dimensions();
                            // let w = dimensions.0;
                            // let h = dimensions.1;
                            // println!("尺寸：{} * {}", w, h);
                            // let small = image.resize(200, 200 / w * h, FilterType::Nearest);
                            // small.save(
                            //     format!(
                            //         "/home/haoran/temp/{}.png",
                            //         path.file_name().unwrap().to_str().unwrap()
                            //     )
                            // );
                            // println!("缩咯图：{} * {}", dimensions.0, dimensions.1);
                        }
                        return true;
                    }
                    _ => {}
                }
            }
        }
        false
    }
}
