extern crate serde;
use serde::ser::{Serialize, SerializeSeq, Serializer};

use std::fmt;
use serde::de::{Deserialize, Deserializer, Error as DeserializeError, Visitor};
use event::EventType;

const ENTRY_LENGTH: usize = 3;

#[derive(Debug)]
pub struct Entry {
    time: f64,
    event_type: EventType,
    event_data: String,
}

impl Serialize for Entry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // This looks silly but `asciicast` uses json arrays instead of
        // objects.
        let mut seq = serializer.serialize_seq(Some(ENTRY_LENGTH))?;
        seq.serialize_element(&self.time)?;
        seq.serialize_element(&self.event_type)?;
        seq.serialize_element(&self.event_data)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Entry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(EntryVisitor)
    }
}

struct EntryVisitor;

impl<'de> Visitor<'de> for EntryVisitor {
    type Value = Entry;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an asciicast entry array")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let time = match seq.next_element::<f64>()? {
            Some(x) => x,
            None => return Err(DeserializeError::invalid_length(0, &self)),
        };
        let event_type = match seq.next_element::<EventType>()? {
            Some(x) => x,
            None => return Err(DeserializeError::invalid_length(1, &self)),
        };
        let event_data = match seq.next_element::<String>()? {
            Some(x) => x,
            None => return Err(DeserializeError::invalid_length(2, &self)),
        };

        Ok(Entry {
            time,
            event_type,
            event_data,
        })
    }
}
