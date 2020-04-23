extern crate image;
use image::{GenericImageView, imageops, Pixel};

use std::fs;
use std::io;
use std::path::{
    Path,
    PathBuf,
};
use std::result;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    _ErrorOpeningFile(PathBuf),
    ExtensionNotFound(PathBuf),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}

pub type Result<T> = result::Result<T, Error>;

pub fn run(input: &str, watermark: &str) -> Result<i32> {
    let _meta = fs::metadata(input)?;
    let _meta_watermark = fs::metadata(watermark)?;
    let path = Path::new(input);
    let path_watermark = Path::new(watermark);
    match path.is_dir() {
        true => walk_folder(&path, &path_watermark),
        false => process_file(&path, &path_watermark)
    }
}

pub fn walk_folder(foldername: &Path, path_watermark: &Path) -> Result<i32> {
    if foldername.is_dir() {
        for entry in fs::read_dir(foldername).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let _res = walk_folder(&path, path_watermark);
            }
            else {
                match process_file(&path, path_watermark) {
                    Ok(_) => continue,
                    Err(e) => eprintln!("{:?}", e)
                }
            }
        }
    }
    Ok(0)
}

pub fn process_file(filename: &Path, path_watermark: &Path) -> Result<i32> {
    let acceptable_extensions = ["png", "jpg"];
    let extension = match filename.extension() {
        Some(ext) => ext,
        None => return Err(Error::ExtensionNotFound(filename.to_path_buf()))
    };
    if acceptable_extensions.contains(&extension.to_str().unwrap()) {
        let _res = watermark_file(&filename, path_watermark);
    }
    Ok(0)
}

pub fn watermark_file(filename: &Path, path_watermark: &Path) -> Result<i32> {
    const WATERMARK_COUNT: u32 = 20; //TODO Let the user specify this
    const WATERMARK_OPACITY: u8 = 50;

    //TODO Watermark is too small on smaller images. Will need to change it so the watermark count is less on small photos

    let name = filename.file_name().unwrap().to_str().unwrap();
    let mut img = image::open(filename).unwrap().into_rgba();
    let (width, height) = img.dimensions();

    let mut watermark = image::open(path_watermark).unwrap();
    let watermark_width: u32 = width / WATERMARK_COUNT;
    let watermark_height: u32 = height / WATERMARK_COUNT;
    watermark = watermark.resize(watermark_width, watermark_height, imageops::FilterType::Triangle);

    for i in 0..WATERMARK_COUNT {
        for j in 0..WATERMARK_COUNT {
            let starting_coord = (width * i / WATERMARK_COUNT, height * j / WATERMARK_COUNT);
            for x in 0..watermark.dimensions().0 {
                for y in 0..watermark.dimensions().1 {
                    let img_x_coord = starting_coord.0 + x;
                    let img_y_coord = starting_coord.1 + y;
                    let mut pixel_watermark = watermark.get_pixel(x, y).clone();
                    let mut img_pixel = img.get_pixel_mut(img_x_coord, img_y_coord).clone();
                    pixel_watermark.0[3] = WATERMARK_OPACITY;
                    img_pixel.blend(&pixel_watermark);
                    img.put_pixel(img_x_coord, img_y_coord, img_pixel);
                }
            }
        }
    }

    fs::create_dir_all("watermarked_photos")?;

    let foo = format!("watermarked_photos/{}{}", "2_", name);
    println!("{}", foo);
    let _res = img.save(foo);
    Ok(0)
}