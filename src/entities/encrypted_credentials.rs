use serde::{Deserialize, Serialize};

///Describes data required for decrypting and authenticating [EncryptedPassportElement](https://core.telegram.org/bots/api/#encryptedpassportelement). See the [Telegram Passport Documentation](https://core.telegram.org/passport#receiving-information) for a complete description of the data decryption and authentication processes.
///
///API Reference: [link](https://core.telegram.org/bots/api/#encryptedcredentials)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EncryptedCredentials {
    ///Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for [EncryptedPassportElement](https://core.telegram.org/bots/api/#encryptedpassportelement) decryption and authentication
    pub data: String,

    ///Base64-encoded data hash for data authentication
    pub hash: String,

    ///Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption
    pub secret: String,
}
// Divider: all content below this line will be preserved after code regen
