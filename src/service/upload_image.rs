use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use image_compressor::compressor::Compressor;
use image_compressor::Factor;
use log::{error, info};
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
use std::{
    error::Error,
    path::{Path, PathBuf},
};
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use uuid::Uuid;
use slug::slugify;

use crate::constant::{TEMP_PATH, ASSETS_PATH};


pub async fn upload_images(mut payload: Multipart) -> Result<(), Box<dyn Error>> {
    let max_file_count: usize = 3;
    // let max_file_size: usize = 100_000;
    let legal_filetypes: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];
    let mut current_count: usize = 0;
    let temp_folder: String = format!("./{}", TEMP_PATH);
    let assets_folder: String = format!("./{}", ASSETS_PATH);

    if !Path::new(&temp_folder.as_str()).exists() {
        match fs::create_dir(&temp_folder.as_str()).await {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    if !Path::new(&assets_folder.as_str()).exists() {
        match fs::create_dir(&assets_folder.as_str()).await {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    let mut error: Option<Box<dyn Error>> = None;

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
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }

            let dest_img = PathBuf::from(&assets_folder);
            let origin_img = PathBuf::from(&uploaded_img);

            let mut comp = Compressor::new(origin_img, dest_img);
            comp.set_delete_origin(true);
            comp.set_factor(Factor::new(75., 0.7));
            match comp.compress_to_jpg() {
                Ok(compressed_img) => {
                    info!("{:?}", compressed_img)
                }
                Err(e) => {
                    error!("Failed {:?}", e);
                    error = Some(e);
                    break;
                }
            };
        } else {
            break;
        }
        current_count += 1;
    }

    match error {
        Some(err) => Err(err)?,
        None => Ok(()),
    }
}
