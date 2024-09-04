use crate::entities::passport_file::PassportFile;
use serde::{Deserialize, Serialize};

///Describes documents or other Telegram Passport elements shared with the bot by the user.
///
///API Reference: [link](https://core.telegram.org/bots/api/#encryptedpassportelement)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EncryptedPassportElement {
    ///Element type. One of “personal\_details”, “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “address”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”, “phone\_number”, “email”.
    #[serde(rename = "type")]
    pub type_: EncryptedPassportElementType,

    ///*Optional*. Base64-encoded encrypted Telegram Passport element data provided by the user; available only for “personal\_details”, “passport”, “driver\_license”, “identity\_card”, “internal\_passport” and “address” types. Can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    ///*Optional*. User's verified phone number; available only for “phone\_number” type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    ///*Optional*. User's verified email address; available only for “email” type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    ///*Optional*. Array of encrypted files with documents provided by the user; available only for “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration” and “temporary\_registration” types. Files can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).
    #[serde(default)]
    pub files: Vec<PassportFile>,

    ///*Optional*. Encrypted file with the front side of the document, provided by the user; available only for “passport”, “driver\_license”, “identity\_card” and “internal\_passport”. The file can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,

    ///*Optional*. Encrypted file with the reverse side of the document, provided by the user; available only for “driver\_license” and “identity\_card”. The file can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,

    ///*Optional*. Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for “passport”, “driver\_license”, “identity\_card” and “internal\_passport”. The file can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,

    ///*Optional*. Array of encrypted files with translated versions of documents provided by the user; available if requested for “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration” and “temporary\_registration” types. Files can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).
    #[serde(default)]
    pub translation: Vec<PassportFile>,

    ///Base64-encoded element hash for using in [PassportElementErrorUnspecified](https://core.telegram.org/bots/api/#passportelementerrorunspecified)
    pub hash: String,
}

///Element type. One of “personal\_details”, “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “address”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”, “phone\_number”, “email”.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "type")]
pub enum EncryptedPassportElementType {
    #[default]
    /// "personal_details"
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

    /// "phone_number"
    #[serde(rename = "phone_number")]
    PhoneNumber,

    /// "email"
    #[serde(rename = "email")]
    Email,
}
// Divider: all content below this line will be preserved after code regen
