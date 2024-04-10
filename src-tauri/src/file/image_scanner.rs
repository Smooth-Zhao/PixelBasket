use std::error::Error;
use std::io::Cursor;
use std::path::Path;

use base64::{engine::general_purpose, Engine as _};
use chrono::offset::Utc;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageFormat};
use kmeans_colors::{get_kmeans_hamerly, Calculate, CentroidData, Sort};
use palette::cast::ComponentsAs;
use palette::{FromColor, IntoColor, Srgb, Srgba};
use sqlx::query;

use crate::db::sqlite::Session;
use crate::db::utils::id;
use crate::file::metadata::{ImageMetadata, Metadata};
use crate::file::scan::Scanner;

pub struct ImageScanner {}

impl ImageScanner {
    pub fn wrap() -> Box<Self> {
        Box::new(ImageScanner {})
    }
}

impl Scanner for ImageScanner {
    fn scan(&self, path: &Path) -> Result<bool, Box<dyn Error>> {
        let mut metadata = Metadata::load(path);
        match metadata.file_suffix.as_str() {
            "avif" | "bmp" | "dds" | "farbfeld" | "gif" | "hdr" | "ico" | "jpg" | "jpeg"
            | "exr" | "png" | "pnm" | "qoi" | "tga" | "tiff" | "webp" => {
                metadata.analyze_metadata(path)?;
                let mut image_metadata = ImageMetadata::new(metadata);
                analyze_image_metadata(path, &mut image_metadata)?;
                save_to_db(image_metadata);
                return Ok(true);
            }
            _ => {}
        }
        Ok(false)
    }
}

/// 解析图片元数据
fn analyze_image_metadata(
    path: &Path,
    image_metadata: &mut ImageMetadata,
) -> Result<(), Box<dyn Error>> {
    let image = image::open(path)?;
    let dimensions = image.dimensions();
    image_metadata.image_width = dimensions.0;
    image_metadata.image_height = dimensions.1;
    let resize_image = resize(
        &image,
        image_metadata.image_width,
        image_metadata.image_height,
    );
    if let Some(base64) = image_to_base64(&resize_image) {
        image_metadata.thumbnail = base64;
    }
    image_metadata.colors = kmeans(&resize_image);
    image_metadata.shape =
        calculated_shape(image_metadata.image_width, image_metadata.image_height);
    Ok(())
}

/// 生成图片缩咯图
fn resize(image: &DynamicImage, w: u32, h: u32) -> DynamicImage {
    let w1 = 200;
    let h1 = (200f32 / w as f32 * h as f32) as u32;
    image.resize(w1, h1, FilterType::Triangle)
}

/// 生成base64图片
fn image_to_base64(image: &DynamicImage) -> Option<String> {
    let mut buffer = Vec::new();
    if image
        .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Jpeg)
        .is_ok()
    {
        let base64 = general_purpose::STANDARD.encode(&buffer);
        return Some(format!("data:image/jpg;base64,{}", base64));
    }
    None
}

/// 提取主题色
fn kmeans(image: &DynamicImage) -> String {
    let img = image.to_rgba8();
    let img_vec: &[Srgba<u8>] = img.as_raw().components_as();

    let mut rgb_pixels: Vec<Srgb<f32>> = Vec::new();
    rgb_pixels.extend(
        img_vec
            .iter()
            .map(|x| Srgb::<f32>::from_color(x.into_format::<_, f32>())),
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

fn save_to_db(image_metadata: ImageMetadata) {
    tokio::spawn(async move {
        let mut session = Session::new("./db/main.db");
        session.connect().await;
        if let Ok(pool) = &session.get_pool() {
            if let Ok(result) = session
                .count(
                    format!(
                        "SELECT COUNT(*) AS count FROM metadata WHERE sha1 = '{}'",
                        &image_metadata.metadata.sha1
                    )
                    .as_str(),
                )
                .await
            {
                if result.count == 0 {
                    let _ = query("INSERT INTO metadata (id, file_name, file_path, file_size, file_suffix, added, created, modified, image_width, image_height, thumbnail, tags, exegesis, score, colors, shape, duration, is_del, sha1) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                        .bind(id::<i64>())
                        .bind(&image_metadata.metadata.file_name)
                        .bind(&image_metadata.metadata.file_path)
                        .bind(image_metadata.metadata.file_size as i64)
                        .bind(&image_metadata.metadata.file_suffix)
                        .bind(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
                        .bind(&image_metadata.metadata.created)
                        .bind(&image_metadata.metadata.modified)
                        .bind(image_metadata.image_width)
                        .bind(image_metadata.image_height)
                        .bind(&image_metadata.thumbnail)
                        .bind(&image_metadata.metadata.tags)
                        .bind(&image_metadata.metadata.exegesis)
                        .bind(image_metadata.metadata.score)
                        .bind(&image_metadata.colors)
                        .bind(&image_metadata.shape)
                        .bind(0)
                        .bind(0)
                        .bind(&image_metadata.metadata.sha1)
                        .execute(pool)
                        .await;
                }
            }
        }
    });
}
