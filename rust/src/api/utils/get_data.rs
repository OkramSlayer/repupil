use byteorder::{BigEndian, ReadBytesExt};
use std::fmt;
use std::io::Cursor;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub artist: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub group: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parody {
    pub parody: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub character: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct Tag {
    pub tag: String,
    pub url: String,
    pub female: Option<String>,
    pub male: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub galleryid: String,
    pub url: String,
    pub language_localname: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GalleryFiles {
    pub width: i32,
    pub hash: String,
    pub haswebp: i32,
    pub name: String,
    pub height: i32,
    pub hasavif: i32,
    pub hasavifsmalltn: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct GalleryInfo {
    pub id: String,
    pub title: String,
    pub japanese_title: Option<String>,
    pub language: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub date: String,
    pub artists: Option<Vec<Artist>>,
    pub groups: Option<Vec<Group>>,
    pub parodys: Option<Vec<Parody>>,
    pub tags: Option<Vec<Tag>>,
    pub related: Vec<i32>,
    pub languages: Vec<Language>,
    pub characters: Option<Vec<Character>>,
    pub scene_indexes: Option<Vec<i32>>,
    pub files: Vec<GalleryFiles>,
}

impl GalleryInfo {
    pub fn new_empty() -> GalleryInfo {
        GalleryInfo {
            id: String::new(),
            title: String::new(),
            japanese_title: None,
            language: None,
            type_: String::new(),
            date: String::new(),
            artists: None,
            groups: None,
            parodys: None,
            tags: None,
            related: Vec::new(),
            languages: Vec::new(),
            characters: None,
            scene_indexes: None,
            files: Vec::new(),
        }
    }
}

impl fmt::Display for GalleryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}\nTitle: {}\nJapanese Title: {:?}\nLanguage: {:?}\nType: {}\nDate: {}\nArtists: {:?}\nGroups: {:?}\nParodys: {:?}\nTags: {:?}\nRelated: {:?}\nLanguages: {:?}\nCharacters: {:?}\nScene Indexes: {:?}\nFiles: {:?}",
            self.id,
            self.title,
            self.japanese_title,
            self.language,
            self.type_,
            self.date,
            self.artists,
            self.groups,
            self.parodys,
            self.tags,
            self.related,
            self.languages,
            self.characters,
            self.scene_indexes,
            self.files
        )
    }
}

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

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct TagDeserializeHelper {
            tag: String,
            url: String,
            female: Option<serde_json::Value>,
            male: Option<serde_json::Value>,
        }

        let helper: TagDeserializeHelper = Deserialize::deserialize(deserializer)?;

        // Convertimos los valores de female y male en String, si son enteros los convertimos a String
        let female = match helper.female {
            Some(value) => Some(value.to_string()),
            None => None,
        };

        let male = match helper.male {
            Some(value) => Some(value.to_string()),
            None => None,
        };

        Ok(Tag {
            tag: helper.tag,
            url: helper.url,
            female,
            male,
        })
    }
}

impl<'de> Deserialize<'de> for GalleryInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct GalleryInfoDeserializeHelper {
            id: serde_json::Value,
            title: String,
            japanese_title: Option<String>,
            language: Option<String>,
            #[serde(rename = "type")]
            type_: String,
            date: String,
            artists: Option<Vec<Artist>>,
            groups: Option<Vec<Group>>,
            parodys: Option<Vec<Parody>>,
            tags: Option<Vec<Tag>>,
            related: Vec<i32>,
            languages: Vec<Language>,
            characters: Option<Vec<Character>>,
            scene_indexes: Option<Vec<i32>>,
            files: Vec<GalleryFiles>,
        }

        let helper: GalleryInfoDeserializeHelper = Deserialize::deserialize(deserializer)?;

        Ok(GalleryInfo {
            id: helper.id.to_string(),
            title: helper.title,
            japanese_title: helper.japanese_title,
            language: helper.language,
            type_: helper.type_,
            date: helper.date,
            artists: helper.artists,
            groups: helper.groups,
            parodys: helper.parodys,
            tags: helper.tags,
            related: helper.related,
            languages: helper.languages,
            characters: helper.characters,
            scene_indexes: helper.scene_indexes,
            files: helper.files,
        })
    }
}