use serde::{Deserialize, Serialize};

///Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
///API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorselfie)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorSelfie {
    ///The section of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”
    #[serde(rename = "type")]
    pub type_: PassportElementErrorSelfieType,

    ///Base64-encoded hash of the file with the selfie
    pub file_hash: String,

    ///Error message
    pub message: String,
}

///The section of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "type")]
pub enum PassportElementErrorSelfieType {
    #[default]
    /// "passport"
    #[serde(rename = "passport")]
    Passport,

    /// "driver_license"
    #[serde(rename = "driver_license")]
    DriverLicense,

    /// "identity_card"
    #[serde(rename = "identity_card")]
    IdentityCard,

    /// "internal_passport"
    #[serde(rename = "internal_passport")]
    InternalPassport,
}
// Divider: all content below this line will be preserved after code regen
