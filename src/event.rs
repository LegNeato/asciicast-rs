extern crate serde;
use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer, Error as DeserializeError, Unexpected};

#[derive(Debug)]
pub enum EventType {
    Input,
    Output,
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            EventType::Input => serializer.serialize_char('i'),
            EventType::Output => serializer.serialize_char('o'),
        }
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<EventType, D::Error>
    where
        D: Deserializer<'de>,
    {
        match <char>::deserialize(deserializer) {
            Ok('i') => Ok(EventType::Input),
            Ok('o') => Ok(EventType::Output),
            Ok(x) => Err(DeserializeError::invalid_value(
                Unexpected::Char(x),
                &"an 'i' or 'o'",
            )),
            Err(x) => Err(x),
        }
    }
}
