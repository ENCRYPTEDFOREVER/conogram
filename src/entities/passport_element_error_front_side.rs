use serde::{Deserialize, Serialize};

/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorfrontside)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PassportElementErrorFrontSide {
    /// The section of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”
    #[serde(rename = "type")]
    pub type_: PassportElementErrorFrontSideType,

    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: String,

    /// Error message
    pub message: String,
}

/// The section of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PassportElementErrorFrontSideType {
    /// `passport`
    #[default]
    #[serde(rename = "passport")]
    Passport,

    /// `driver_license`
    #[serde(rename = "driver_license")]
    DriverLicense,

    /// `identity_card`
    #[serde(rename = "identity_card")]
    IdentityCard,

    /// `internal_passport`
    #[serde(rename = "internal_passport")]
    InternalPassport,
}

// Divider: all content below this line will be preserved after code regen
