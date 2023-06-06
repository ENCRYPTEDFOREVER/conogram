use crate::entities::passport_element_error_data_field::PassportElementErrorDataField;
use crate::entities::passport_element_error_file::PassportElementErrorFile;
use crate::entities::passport_element_error_files::PassportElementErrorFiles;
use crate::entities::passport_element_error_front_side::PassportElementErrorFrontSide;
use crate::entities::passport_element_error_reverse_side::PassportElementErrorReverseSide;
use crate::entities::passport_element_error_selfie::PassportElementErrorSelfie;
use crate::entities::passport_element_error_translation_file::PassportElementErrorTranslationFile;
use crate::entities::passport_element_error_translation_files::PassportElementErrorTranslationFiles;
use crate::entities::passport_element_error_unspecified::PassportElementErrorUnspecified;
use serde::{Deserialize, Serialize};

///This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
///
///* [PassportElementErrorDataField](https://core.telegram.org/bots/api/#passportelementerrordatafield)
///* [PassportElementErrorFrontSide](https://core.telegram.org/bots/api/#passportelementerrorfrontside)
///* [PassportElementErrorReverseSide](https://core.telegram.org/bots/api/#passportelementerrorreverseside)
///* [PassportElementErrorSelfie](https://core.telegram.org/bots/api/#passportelementerrorselfie)
///* [PassportElementErrorFile](https://core.telegram.org/bots/api/#passportelementerrorfile)
///* [PassportElementErrorFiles](https://core.telegram.org/bots/api/#passportelementerrorfiles)
///* [PassportElementErrorTranslationFile](https://core.telegram.org/bots/api/#passportelementerrortranslationfile)
///* [PassportElementErrorTranslationFiles](https://core.telegram.org/bots/api/#passportelementerrortranslationfiles)
///* [PassportElementErrorUnspecified](https://core.telegram.org/bots/api/#passportelementerrorunspecified)
///API Reference: [link](https://core.telegram.org/bots/api/#passportelementerror)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum PassportElementError {
    #[serde(rename = "data")]
    DataField(PassportElementErrorDataField),
    #[serde(rename = "front_side")]
    FrontSide(PassportElementErrorFrontSide),
    #[serde(rename = "reverse_side")]
    ReverseSide(PassportElementErrorReverseSide),
    #[serde(rename = "selfie")]
    Selfie(PassportElementErrorSelfie),
    #[serde(rename = "file")]
    File(PassportElementErrorFile),
    #[serde(rename = "files")]
    Files(PassportElementErrorFiles),
    #[serde(rename = "translation_file")]
    TranslationFile(PassportElementErrorTranslationFile),
    #[serde(rename = "translation_files")]
    TranslationFiles(PassportElementErrorTranslationFiles),
    #[serde(rename = "unspecified")]
    Unspecified(PassportElementErrorUnspecified),
}
// Divider: all content below this line will be preserved after code regen
