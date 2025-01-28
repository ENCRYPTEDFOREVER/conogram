use serde::{Deserialize, Serialize};

use crate::entities::passport_element_error_translation_file::PassportElementErrorTranslationFileType;

/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrortranslationfiles)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    /// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
    #[serde(rename = "type")]
    pub type_: PassportElementErrorTranslationFileType,

    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,

    /// Error message
    pub message: String,
}

// Divider: all content below this line will be preserved after code regen
