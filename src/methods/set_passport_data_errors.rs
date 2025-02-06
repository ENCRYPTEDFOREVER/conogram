use conogram_derives::Request;
use serde::Serialize;

use crate::entities::passport_element_error::PassportElementError;

/// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns *True* on success.
///
/// Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setpassportdataerrors)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetPassportDataErrorsParams {
    /// User identifier
    pub user_id: i64,

    /// A JSON-serialized array describing the errors
    pub errors: Vec<PassportElementError>,
}

// Divider: all content below this line will be preserved after code regen
