use crate::api::utils::get_data::get_data_from_url;
use crate::api::utils::generate_url::generate_url_by_area;

use super::models::models::GalleryInfo;
use super::utils::generate_url::generate_url_by_gallery_id;
use super::utils::get_data::get_gallery_data_from_url;

pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

pub async fn get_gallery_ids_from_nozomi(
    area: Option<String>, 
    tag: String, //index, popular
    language: String,
) -> Vec<i32> {
    let nozomi_address: String = generate_url_by_area(area, tag, language);
    let gallery_ids: Vec<i32> = get_data_from_url(nozomi_address).await;
    return gallery_ids;
}

pub async fn get_gallery_data_from_id(gallery_id: u32) -> GalleryInfo {
    let gallery_url: String = generate_url_by_gallery_id(gallery_id);
    let gallery_data = get_gallery_data_from_url(gallery_url).await;
    if gallery_data.id.eq(&String::from("")) {
        eprintln!("Gallery with id: {} is empty", gallery_id);
    } else {
        // Get cover
    }
    return gallery_data;
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
