use conogram_derives::Request;
use serde::Serialize;

use crate::entities::bot_name::BotName;

/// Use this method to get the current bot name for the given user language. Returns [BotName](https://core.telegram.org/bots/api/#botname) on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getmyname)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = BotName)]
pub struct GetMyNameParams {
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
