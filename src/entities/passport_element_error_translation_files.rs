use serde::{Deserialize, Serialize};

///Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
///API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrortranslationfiles)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    ///Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
    #[serde(rename = "type")]
    pub type_: PassportElementErrorTranslationFilesType,

    ///List of base64-encoded file hashes
    pub file_hashes: Vec<String>,

    ///Error message
    pub message: String,
}

///Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "type")]
pub enum PassportElementErrorTranslationFilesType {
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
