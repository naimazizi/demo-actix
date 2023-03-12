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

pub async fn upload_images(mut payload: Multipart) -> Result<(), Box<dyn Error>> {
    let max_file_count: usize = 3;
    // let max_file_size: usize = 100_000;
    let legal_filetypes: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];
    let mut current_count: usize = 0;
    let dir: &str = "./.upload/";

    if !Path::new("./.upload").exists() {
        match fs::create_dir("./.upload").await {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    if !Path::new("./.compressed").exists() {
        match fs::create_dir("./.compressed").await {
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

            let destination: String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap()
            );

            let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }

            let destination_path = Path::new(&destination);
            let compressed_file_path = format!(
                "{}/.compressed",
                destination_path
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            );

            let dest = PathBuf::from(compressed_file_path);
            let origin = PathBuf::from(&destination);

            let mut comp = Compressor::new(origin, dest);
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
