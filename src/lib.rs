mod entry;
mod event;
mod header;
pub use entry::*;
pub use event::*;
pub use header::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
enum EnvVar {
    Shell,
    Term,
    Custom(String),
}
