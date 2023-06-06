use serde::{Deserialize, Serialize};

///Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
///API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorfiles)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFiles {
    ///The section of the user's Telegram Passport which has the issue, one of “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
    #[serde(rename = "type")]
    pub type_: PassportElementErrorFilesType,

    ///List of base64-encoded file hashes
    pub file_hashes: Vec<String>,

    ///Error message
    pub message: String,
}

///The section of the user's Telegram Passport which has the issue, one of “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "type")]
pub enum PassportElementErrorFilesType {
    #[default]
    /// "utility_bill"
    #[serde(rename = "utility_bill")]
    UtilityBill,

    /// "bank_statement"
    #[serde(rename = "bank_statement")]
    BankStatement,

    /// "rental_agreement"
    #[serde(rename = "rental_agreement")]
    RentalAgreement,

    /// "passport_registration"
    #[serde(rename = "passport_registration")]
    PassportRegistration,

    /// "temporary_registration"
    #[serde(rename = "temporary_registration")]
    TemporaryRegistration,
}
// Divider: all content below this line will be preserved after code regen
