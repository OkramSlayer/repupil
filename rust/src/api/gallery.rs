use std::collections::HashSet;
use crate::api::utils::get_data::get_data_from_url;
use crate::api::utils::generate_url::generate_url_by_area;

pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

pub async fn get_gallery_ids_from_nozomi(
    area: Option<String>, 
    tag: String, //index, popular
    language: String,
) -> HashSet<i32> {
    let nozomi_address: String = generate_url_by_area(area, tag, language);
    let gallery_ids: HashSet<i32> = get_data_from_url(nozomi_address).await;
    return gallery_ids;
}

//https://ltn.hitomi.la/galleries/2819978.js

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
