use serde::{Deserialize, Serialize};

/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrortranslationfile)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFile {
    /// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
    #[serde(rename = "type")]
    pub type_: PassportElementErrorTranslationFileType,

    /// Base64-encoded file hash
    pub file_hash: String,

    /// Error message
    pub message: String,
}

/// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PassportElementErrorTranslationFileType {
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

    /// `utility_bill`
    #[serde(rename = "utility_bill")]
    UtilityBill,

    /// `bank_statement`
    #[serde(rename = "bank_statement")]
    BankStatement,

    /// `rental_agreement`
    #[serde(rename = "rental_agreement")]
    RentalAgreement,

    /// `passport_registration`
    #[serde(rename = "passport_registration")]
    PassportRegistration,

    /// `temporary_registration`
    #[serde(rename = "temporary_registration")]
    TemporaryRegistration,
}

// Divider: all content below this line will be preserved after code regen
