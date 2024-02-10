use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

use crate::api::models::models::GalleryInfo;

pub async fn get_data_from_url(nozomi_address: String) -> Vec<i32> {
    let response = match reqwest::get(&nozomi_address).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Vec::new();
        }
    };

    let body = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error reading response: {}", e);
            return Vec::new();
        }
    };

    let mut nozomi = Vec::new();
    let mut cursor = Cursor::new(body);

    while let Ok(value) = cursor.read_i32::<BigEndian>(){
        nozomi.push(value);
    }

    return nozomi;
}

pub async fn get_gallery_data_from_url(gallery_url: String) -> GalleryInfo {

    let response = match reqwest::get(gallery_url).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: {}", e);
            return GalleryInfo::new_empty();
        }
    };

    let body = match response.text().await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error reading response: {}", e);
            return GalleryInfo::new_empty();
        }
    };

    let trimmed_response = body.trim_start_matches("var galleryinfo = ");

    let gallery: GalleryInfo = match serde_json::from_str(trimmed_response) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return GalleryInfo::new_empty();
        }
    };

    return gallery;
}
