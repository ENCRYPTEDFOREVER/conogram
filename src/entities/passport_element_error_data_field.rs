use serde::{Deserialize, Serialize};

/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrordatafield)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorDataField {
    /// The section of the user's Telegram Passport which has the error, one of “personal\_details”, “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “address”
    #[serde(rename = "type")]
    pub type_: PassportElementErrorDataFieldType,

    /// Name of the data field which has the error
    pub field_name: String,

    /// Base64-encoded data hash
    pub data_hash: String,

    /// Error message
    pub message: String,
}

/// The section of the user's Telegram Passport which has the error, one of “personal\_details”, “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “address”
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub enum PassportElementErrorDataFieldType {
    /// "personal_details"
    #[default]
    #[serde(rename = "personal_details")]
    PersonalDetails,

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

    /// "address"
    #[serde(rename = "address")]
    Address,
}

// Divider: all content below this line will be preserved after code regen
