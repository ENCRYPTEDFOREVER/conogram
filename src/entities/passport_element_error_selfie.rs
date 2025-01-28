use serde::{Deserialize, Serialize};

use crate::entities::passport_element_error_front_side::PassportElementErrorFrontSideType;

/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorselfie)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PassportElementErrorSelfie {
    /// The section of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”
    #[serde(rename = "type")]
    pub type_: PassportElementErrorFrontSideType,

    /// Base64-encoded hash of the file with the selfie
    pub file_hash: String,

    /// Error message
    pub message: String,
}

// Divider: all content below this line will be preserved after code regen
