extern crate serde;

#[cfg(feature = "chrono")]
extern crate chrono;
#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    version: u8,
    width: u32,
    height: u32,
    #[cfg(feature = "chrono")]
    #[serde(skip_serializing_if = "Option::is_none", with = "timestamp_format")]
    timestamp: Option<DateTime<Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_time_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    // TODO: env.
    //env: HashMap<EnvVar, Option<String>>,
    // TODO: Theme.
}

#[cfg(feature = "chrono")]
mod timestamp_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            &Some(x) => serializer.serialize_i64(x.timestamp()),
            &None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match <i64>::deserialize(deserializer) {
            Ok(timestamp) => Ok(Some(DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(timestamp, 0),
                Utc,
            ))),
            // TODO: Should we return an error here? Not sure how deserializers
            // work in serde.
            _ => Ok(None),
        }
    }
}
