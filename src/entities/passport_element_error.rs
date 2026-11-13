use serde::{Deserialize, Serialize};

use crate::entities::{
    passport_element_error_data_field::PassportElementErrorDataField,
    passport_element_error_file::PassportElementErrorFile,
    passport_element_error_files::PassportElementErrorFiles,
    passport_element_error_front_side::PassportElementErrorFrontSide,
    passport_element_error_reverse_side::PassportElementErrorReverseSide,
    passport_element_error_selfie::PassportElementErrorSelfie,
    passport_element_error_translation_file::PassportElementErrorTranslationFile,
    passport_element_error_translation_files::PassportElementErrorTranslationFiles,
    passport_element_error_unspecified::PassportElementErrorUnspecified,
};

/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
///
/// * [PassportElementErrorDataField](https://core.telegram.org/bots/api/#passportelementerrordatafield)
/// * [PassportElementErrorFrontSide](https://core.telegram.org/bots/api/#passportelementerrorfrontside)
/// * [PassportElementErrorReverseSide](https://core.telegram.org/bots/api/#passportelementerrorreverseside)
/// * [PassportElementErrorSelfie](https://core.telegram.org/bots/api/#passportelementerrorselfie)
/// * [PassportElementErrorFile](https://core.telegram.org/bots/api/#passportelementerrorfile)
/// * [PassportElementErrorFiles](https://core.telegram.org/bots/api/#passportelementerrorfiles)
/// * [PassportElementErrorTranslationFile](https://core.telegram.org/bots/api/#passportelementerrortranslationfile)
/// * [PassportElementErrorTranslationFiles](https://core.telegram.org/bots/api/#passportelementerrortranslationfiles)
/// * [PassportElementErrorUnspecified](https://core.telegram.org/bots/api/#passportelementerrorunspecified)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerror)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum PassportElementError {
    /// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrordatafield)
    #[serde(rename = "data")]
    DataField(PassportElementErrorDataField),

    /// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorfrontside)
    #[serde(rename = "front_side")]
    FrontSide(PassportElementErrorFrontSide),

    /// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorreverseside)
    #[serde(rename = "reverse_side")]
    ReverseSide(PassportElementErrorReverseSide),

    /// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorselfie)
    #[serde(rename = "selfie")]
    Selfie(PassportElementErrorSelfie),

    /// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorfile)
    #[serde(rename = "file")]
    File(PassportElementErrorFile),

    /// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorfiles)
    #[serde(rename = "files")]
    Files(PassportElementErrorFiles),

    /// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrortranslationfile)
    #[serde(rename = "translation_file")]
    TranslationFile(PassportElementErrorTranslationFile),

    /// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrortranslationfiles)
    #[serde(rename = "translation_files")]
    TranslationFiles(PassportElementErrorTranslationFiles),

    /// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#passportelementerrorunspecified)
    #[serde(rename = "unspecified")]
    Unspecified(PassportElementErrorUnspecified),
}

impl Default for PassportElementError {
    fn default() -> Self {
        Self::DataField(PassportElementErrorDataField::default())
    }
}

impl From<PassportElementErrorDataField> for PassportElementError {
    fn from(value: PassportElementErrorDataField) -> Self {
        Self::DataField(value)
    }
}

impl From<PassportElementErrorFrontSide> for PassportElementError {
    fn from(value: PassportElementErrorFrontSide) -> Self {
        Self::FrontSide(value)
    }
}

impl From<PassportElementErrorReverseSide> for PassportElementError {
    fn from(value: PassportElementErrorReverseSide) -> Self {
        Self::ReverseSide(value)
    }
}

impl From<PassportElementErrorSelfie> for PassportElementError {
    fn from(value: PassportElementErrorSelfie) -> Self {
        Self::Selfie(value)
    }
}

impl From<PassportElementErrorFile> for PassportElementError {
    fn from(value: PassportElementErrorFile) -> Self {
        Self::File(value)
    }
}

impl From<PassportElementErrorFiles> for PassportElementError {
    fn from(value: PassportElementErrorFiles) -> Self {
        Self::Files(value)
    }
}

impl From<PassportElementErrorTranslationFile> for PassportElementError {
    fn from(value: PassportElementErrorTranslationFile) -> Self {
        Self::TranslationFile(value)
    }
}

impl From<PassportElementErrorTranslationFiles> for PassportElementError {
    fn from(value: PassportElementErrorTranslationFiles) -> Self {
        Self::TranslationFiles(value)
    }
}

impl From<PassportElementErrorUnspecified> for PassportElementError {
    fn from(value: PassportElementErrorUnspecified) -> Self {
        Self::Unspecified(value)
    }
}

// Divider: all content below this line will be preserved after code regen
