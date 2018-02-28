extern crate serde;

#[cfg(feature = "chrono")]
extern crate chrono;
#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Header {
    pub version: u8,
    pub width: u32,
    pub height: u32,
    #[cfg(feature = "chrono")]
    #[serde(skip_serializing_if = "Option::is_none", with = "timestamp_format")]
    pub timestamp: Option<DateTime<Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_time_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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

#[cfg(test)]
mod tests {
    use serde_json;
    use super::Header;

    #[test]
    fn test_deserializes_with_optional_data() {
        // TODO: Figure out why with chrono integration we need this extra
        // `timestamp`. Using `cargo test --no-default-features` works fine.
        let data = r#"{"version":2, "width":80, "height":40, "timestamp": null}"#;
        let h: Header = serde_json::from_str(data).unwrap();
        assert_eq!(
            h,
            Header {
                version: 2,
                width: 80,
                height: 40,
                timestamp: None,
                duration: None,
                idle_time_limit: None,
                command: None,
                title: None,
            }
        );
    }

    #[test]
    fn test_serializes_with_optional_data() {
        let h = Header {
            version: 2,
            width: 42,
            height: 100,
            timestamp: None,
            duration: None,
            idle_time_limit: None,
            command: None,
            title: None,
        };

        // Serialize it to a JSON string.
        let result = serde_json::to_string(&h).unwrap();
        assert_eq!(result, r#"{"version":2,"width":42,"height":100}"#);
    }
}
