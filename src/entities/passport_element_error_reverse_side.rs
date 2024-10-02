use serde::{Deserialize, Serialize};

/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorreverseside)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorReverseSide {
    /// The section of the user's Telegram Passport which has the issue, one of “driver\_license”, “identity\_card”
    #[serde(rename = "type")]
    pub type_: PassportElementErrorReverseSideType,

    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,

    /// Error message
    pub message: String,
}

/// The section of the user's Telegram Passport which has the issue, one of “driver\_license”, “identity\_card”
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub enum PassportElementErrorReverseSideType {
    /// "driver_license"
    #[serde(rename = "driver_license")]
    #[default]
    DriverLicense,

    /// "identity_card"
    #[serde(rename = "identity_card")]
    IdentityCard,
}

// Divider: all content below this line will be preserved after code regen
