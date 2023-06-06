use serde::{Deserialize, Serialize};

///Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
///API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorfile)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFile {
    ///The section of the user's Telegram Passport which has the issue, one of “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
    #[serde(rename = "type")]
    pub type_: PassportElementErrorFileType,

    ///Base64-encoded file hash
    pub file_hash: String,

    ///Error message
    pub message: String,
}

///The section of the user's Telegram Passport which has the issue, one of “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "type")]
pub enum PassportElementErrorFileType {
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
