use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use image_compressor::compressor::Compressor;
use image_compressor::Factor;
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
use slug::slugify;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use uuid::Uuid;

use crate::constant::{ASSETS_PATH, TEMP_PATH};

pub async fn upload_images(mut payload: Multipart) -> Result<(), std::io::Error> {
    let max_file_count: usize = 3;
    // let max_file_size: usize = 100_000;
    let legal_filetypes: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];
    let mut current_count: usize = 0;
    let temp_folder: String = format!("./{}", TEMP_PATH);
    let assets_folder: String = format!("./{}", ASSETS_PATH);

    if !Path::new(&temp_folder.as_str()).exists() {
        fs::create_dir(&temp_folder.as_str()).await?
    }

    if !Path::new(&assets_folder.as_str()).exists() {
        fs::create_dir(&assets_folder.as_str()).await?
    }

    loop {
        if current_count == max_file_count {
            break;
        }
        if let Ok(Some(mut field)) = payload.try_next().await {
            let filetype: Option<&Mime> = field.content_type();
            if filetype.is_none() {
                continue;
            }
            if !legal_filetypes.contains(&filetype.unwrap()) {
                continue;
            }

            let img_name = field.content_disposition().get_filename().unwrap();
            let img_path = Path::new(&img_name);
            let img_ext = img_path.extension().unwrap().to_str().unwrap();

            let uploaded_img: String = format!(
                "{}/{}-{}.{}",
                &temp_folder.as_str(),
                Uuid::new_v4(),
                slugify(img_path.file_stem().unwrap().to_str().unwrap()),
                img_ext
            );

            let mut saved_file: fs::File = fs::File::create(&uploaded_img).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = match saved_file.write_all(&chunk).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                };
            }

            let dest_img = PathBuf::from(&assets_folder);
            let origin_img = PathBuf::from(&uploaded_img);

            let mut comp = Compressor::new(origin_img, dest_img);
            comp.set_delete_origin(true);
            comp.set_factor(Factor::new(75., 0.7));
            let _ = comp.compress_to_jpg();
        } else {
            break;
        }
        current_count += 1;
    }

    Ok(())
}
