use serde::{Deserialize, Serialize};

use crate::entities::{
    encrypted_credentials::EncryptedCredentials,
    encrypted_passport_element::EncryptedPassportElement,
};

/// Describes Telegram Passport data shared with the bot by the user.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#passportdata)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport elements that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,

    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}

// Divider: all content below this line will be preserved after code regen
