mod entry;
mod event;
mod header;
pub use entry::*;
pub use event::*;
pub use header::*;

#[cfg(feature = "chrono")]
extern crate chrono;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
enum EnvVar {
    SHELL,
    TERM,
    Custom(String),
}
