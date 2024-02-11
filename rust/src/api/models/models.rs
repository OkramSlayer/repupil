
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
    pub cover_url: Option<String>,
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
            cover_url: None,
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
          cover_url: None,
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