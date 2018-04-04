extern crate serde;

#[cfg(feature = "chrono")]
extern crate chrono;
#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Header {
    /// `asciicast` format version.
    pub version: u8,
    /// Initial terminal width (number of columns).
    pub width: u32,
    /// Initial terminal height (number of rows).
    pub height: u32,
    #[cfg(feature = "chrono")]
    #[serde(skip_serializing_if = "Option::is_none", with = "timestamp_format")]
    /// Time of the beginning of the recording session.
    pub timestamp: Option<DateTime<Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unix timestamp of the beginning of the recording session.
    pub timestamp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration of the whole recording in seconds (when it's known upfront).
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Idle time limit, as given via `-i` option to `asciinema rec`.
    /// This should be used by an asciicast player to reduce all terminal inactivity
    /// (delays between frames) to maximum of `idle_time_limit` value.
    pub idle_time_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Command that was recorded, as given via `-c` option to `asciinema rec`.
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Title of the asciicast, as given via `-t` option to `asciinema rec`.
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Map of captured environment variables.
    pub env: Option<HashMap<String, String>>,
    // TODO: theme.
}

#[cfg(feature = "chrono")]
mod timestamp_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *date {
            Some(x) => serializer.serialize_i64(x.timestamp()),
            None => serializer.serialize_none(),
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
    use std::collections::HashMap;

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
                env: None,
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
            env: None,
        };

        // Serialize it to a JSON string.
        let result = serde_json::to_string(&h).unwrap();
        assert_eq!(result, r#"{"version":2,"width":42,"height":100}"#);
    }

    #[test]
    fn test_serializes_env() {
        let mut env = HashMap::new();
        env.insert("FOO".to_string(), "test".to_string());

        let h = Header {
            version: 2,
            width: 42,
            height: 100,
            timestamp: None,
            duration: None,
            idle_time_limit: None,
            command: None,
            title: None,
            env: Some(env),
        };

        // Serialize it to a JSON string.
        let result = serde_json::to_string(&h).unwrap();
        assert_eq!(
            result,
            r#"{"version":2,"width":42,"height":100,"env":{"FOO":"test"}}"#
        );
    }
}
