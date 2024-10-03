use serde::{Deserialize, Serialize};

use crate::entities::passport_element_error_file::PassportElementErrorFileType;

/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorfiles)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFiles {
    /// The section of the user's Telegram Passport which has the issue, one of “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
    #[serde(rename = "type")]
    pub type_: PassportElementErrorFileType,

    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,

    /// Error message
    pub message: String,
}

// Divider: all content below this line will be preserved after code regen
