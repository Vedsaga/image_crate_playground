use std::path::PathBuf;

use exif::{In, Tag};
use image::{imageops, DynamicImage};

pub fn get_jpeg_orientation(file_path: PathBuf) -> Result<u32, ()> {
    let file = std::fs::File::open(file_path).expect("problem opening the file");
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader
        .read_from_container(&mut bufreader)
        .expect("failed to read the exifreader");

    let orientation: u32 = match exif.get_field(Tag::Orientation, In::PRIMARY) {
        Some(orientation) => match orientation.value.get_uint(0) {
            Some(v @ 1..=8) => v,
            _ => 1,
        },
        None => 1,
    };

    Ok(orientation)
}

fn rotate(mut img: DynamicImage, orientation: u8) -> DynamicImage {
    let rgba = img.color().has_alpha();
    img = match orientation {
        2 => DynamicImage::ImageRgba8(imageops::flip_horizontal(&img)),
        3 => DynamicImage::ImageRgba8(imageops::rotate180(&img)),
        4 => DynamicImage::ImageRgba8(imageops::flip_vertical(&img)),
        5 => DynamicImage::ImageRgba8(imageops::flip_horizontal(&imageops::rotate90(&img))),
        6 => DynamicImage::ImageRgba8(imageops::rotate90(&img)),
        7 => DynamicImage::ImageRgba8(imageops::flip_horizontal(&imageops::rotate270(&img))),
        8 => DynamicImage::ImageRgba8(imageops::rotate270(&img)),
        _ => img,
    };
    if !rgba {
        img = DynamicImage::ImageRgb8(img.into_rgb8());
    }
    img
}

fn main() {
    // C:\\Users\\harsh\\Downloads\\test-image-path\\IMG_8146.jpeg",
    let img_path = PathBuf::from("C:\\Users\\harsh\\Downloads\\test-image-path\\IMG_8146.jpeg");
    let _ = match image::open(img_path.to_owned()) {
        Ok(original_image) => {
            // save the image at C:\Users\harsh\Downloads\media_processing_output just add new_ prefix to existing filename
            let path_to_save =
                PathBuf::from("C:\\Users\\harsh\\Downloads\\media_processing_output\\").join(
                    format!("new_{}", img_path.file_name().unwrap().to_str().unwrap()),
                );
                
            let orientation = get_jpeg_orientation(img_path).unwrap();

            let img = rotate(original_image, orientation as u8);

            let _ = img.save(path_to_save.to_owned());

            println!("saved image at {}", path_to_save.display());
        }
        Err(_) => {
            println!("Error loading image");
            return;
        }
    };
}
