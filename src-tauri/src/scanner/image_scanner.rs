use std::ops::Add;
use std::path::{Path, PathBuf};

use image::{DynamicImage, GenericImageView, ImageFormat, RgbImage};
use kmeans_colors::{Calculate, CentroidData, get_kmeans_hamerly, Sort};
use palette::{FromColor, IntoColor, Srgb};
use palette::cast::ComponentsAs;

use crate::data::metadata::Metadata;
use crate::data::task::{Task, TaskStatus};
use crate::Result;
use crate::scanner::scan::{Context, Scanner};
use crate::util::error::ErrorHandle;
use crate::util::snowflake::id_str;

pub struct ImageScanner {}

impl ImageScanner {
    pub fn wrap() -> Box<Self> {
        Box::new(ImageScanner {})
    }
}

impl Scanner for ImageScanner {
    fn is_support(&self, suffix: &str) -> bool {
        match suffix.to_lowercase().as_str() {
            "avif" | "bmp" | "dds" | "farbfeld" | "gif" | "hdr" | "ico" | "jpg" | "jpeg"
            | "exr" | "png" | "pnm" | "qoi" | "tga" | "tiff" | "webp" => true,
            _ => false,
        }
    }

    fn scan(&self, task: &Task, context: &Context) -> TaskStatus {
        let mut status = TaskStatus::new(task.id);
        if self.is_support(task.file_suffix.as_str()) {
            let path = task.file_path.clone();
            let runtime = context.runtime.handle().clone();
            let db_runtime = context.db_runtime.handle().clone();
            let mut cache_path = context.cache_path.clone();
            status.handle(runtime.clone().spawn_blocking(move || {
                let path = Path::new(path.as_str());
                let mut metadata = Metadata::load(path);
                if metadata.analyze_metadata(path).is_ok() {
                    if analyze_image_metadata(path, &mut cache_path, &mut metadata)
                        .print_error()
                        .is_some()
                    {
                        // 使用阻塞线程防止数据丢失！
                        db_runtime.block_on(async move {
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
fn analyze_image_metadata(
    path: &Path,
    cache_path: &mut PathBuf,
    metadata: &mut Metadata,
) -> Result<()> {
    let image = image::open(path)?;
    let dimensions = image.dimensions();
    metadata.image_width = dimensions.0;
    metadata.image_height = dimensions.1;
    let resize_image = if metadata.image_width > 200 {
        thumbnail(&image, metadata.image_width, metadata.image_height)
    } else {
        image.to_rgb8()
    };
    cache_path.push(id_str().add(".thumbnail"));
    resize_image.save_with_format(&cache_path, ImageFormat::Jpeg)?;
    metadata.thumbnail = cache_path.to_string_lossy().to_string();
    metadata.colors = kmeans(&resize_image);
    metadata.shape = calculated_shape(metadata.image_width, metadata.image_height);
    Ok(())
}

/// 生成图片缩咯图
pub fn thumbnail(image: &DynamicImage, w: u32, h: u32) -> RgbImage {
    let w1 = 200;
    let h1 = (200f32 / w as f32 * h as f32) as u32;
    image.thumbnail(w1, h1).to_rgb8()
}

/// 提取主题色
fn kmeans(image: &RgbImage) -> String {
    let img_vec: &[Srgb<u8>] = image.as_raw().components_as();

    let mut rgb_pixels: Vec<Srgb<f32>> = Vec::new();
    rgb_pixels.extend(
        img_vec
            .iter()
            .map(|x| Srgb::<f32>::from_color(x.into_format())),
    );

    let result = get_kmeans_hamerly(8, 1, 0.0025, false, &rgb_pixels, 0);
    let result = Srgb::sort_indexed_colors(&result.centroids, &result.indices);
    to_colors(&result)
}

fn to_colors<C: Calculate + Copy + IntoColor<Srgb>>(colors: &[CentroidData<C>]) -> String {
    colors
        .iter()
        .map(|x| format!("#{:x}", x.centroid.into_color().into_format::<u8>()))
        .collect::<Vec<String>>()
        .join(",")
}

fn calculated_shape(w: u32, h: u32) -> String {
    let divisor = greatest_common_divisor(w, h);
    format!("{}:{}", w / divisor, h / divisor)
}

fn greatest_common_divisor(a: u32, b: u32) -> u32 {
    if a == 0 {
        b
    } else {
        greatest_common_divisor(b % a, a)
    }
}
