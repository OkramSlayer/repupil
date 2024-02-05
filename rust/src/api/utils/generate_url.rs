
pub fn generate_url_by_area(
    area: Option<String>,
    tag: String, //index, popular
    language: String
) -> String {
  let protocol: String = "https".to_owned();
    let domain: String = "ltn.hitomi.la".to_owned();
    let port = 443;
    let compressed_nozomi_prefix: String = "n".to_owned();
    let nozomi_extension : String= ".nozomi".to_owned();

    return match area {
        Some(a) if !a.is_empty() => format!(
            "{}://{}:{}/{}/{}/{}-{}{}",
            protocol, domain, port, compressed_nozomi_prefix, a, tag, language, nozomi_extension
        ),
        Some(_) | None => format!(
            "{}://{}:{}/{}/{}-{}{}",
            protocol, domain, port, compressed_nozomi_prefix, tag, language, nozomi_extension
        ),
    };
} 