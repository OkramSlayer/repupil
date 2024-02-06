static PROTOCOL: &'static str = "https";
static DOMAIN: &'static str = "ltn.hitomi.la";
static PORT: i32 = 443;
static COMPRESSED_NOZOMI_PREFIX: &'static str = "n";
static NOZOMI_EXTENSION : &'static str= ".nozomi";

pub fn generate_url_by_area(
    area: Option<String>,
    tag: String, //index, popular
    language: String
) -> String {
    return match area {
        Some(a) if !a.is_empty() => format!(
            "{}://{}:{}/{}/{}/{}-{}{}",
            PROTOCOL, DOMAIN, PORT, COMPRESSED_NOZOMI_PREFIX, a, tag, language, NOZOMI_EXTENSION
        ),
        Some(_) | None => format!(
            "{}://{}:{}/{}/{}-{}{}",
            PROTOCOL, DOMAIN, PORT, COMPRESSED_NOZOMI_PREFIX, tag, language, NOZOMI_EXTENSION
        ),
    };
}
//https://ltn.hitomi.la/galleries/2819978.js
pub fn generate_url_by_gallery_id(gallery_id: u32) -> String {
    return format!("{}://{}:{}/galleries/{}.js", PROTOCOL, DOMAIN, PORT, gallery_id)
}