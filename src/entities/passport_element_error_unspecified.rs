use serde::{Deserialize, Serialize};

///Represents an issue in an unspecified place. The error is considered resolved when new data is added.
///
///API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorunspecified)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorUnspecified {
    ///Type of element of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub type_: String,

    ///Base64-encoded element hash
    pub element_hash: String,

    ///Error message
    pub message: String,
}
// Divider: all content below this line will be preserved after code regen
