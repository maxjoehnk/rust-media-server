use library::Track;
use provider::Provider;
use std::str::FromStr;
use std::fmt::Display;
use serde::de::{self, Deserialize, Deserializer};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PocketcastEpisode<> {
    pub uuid: String,
    pub size: i32,
    pub title: String,
    pub url: String,
    #[serde(default, with = "string_or_int")]
    pub duration: Option<u64>,
}

impl From<PocketcastEpisode> for Track {
    fn from(episode: PocketcastEpisode) -> Track {
        Track {
            id: None,
            title: episode.title,
            artist_id: None,
            album_id: None,
            stream_url: episode.url,
            provider: Provider::Pocketcasts,
            path: format!("pocketcasts:{}", episode.uuid),
            coverart: None,
            duration: episode.duration,
        }
    }
}

mod string_or_int {
    use std::fmt;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match value {
            &Some(value) => serializer.collect_str(&value),
            &None => serializer.serialize_unit()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt {
            String(String),
            Int(u64),
            Null,
        }

        match StringOrInt::deserialize(deserializer)? {
            StringOrInt::String(s) => s.parse().map_err(de::Error::custom).map(|val| Some(val)),
            StringOrInt::Int(i) => Ok(Some(i)),
            StringOrInt::Null => Ok(None)
        }
    }
}