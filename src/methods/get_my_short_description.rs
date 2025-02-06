use conogram_derives::Request;
use serde::Serialize;

use crate::entities::bot_short_description::BotShortDescription;

/// Use this method to get the current bot short description for the given user language. Returns [BotShortDescription](https://core.telegram.org/bots/api/#botshortdescription) on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getmyshortdescription)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = BotShortDescription)]
pub struct GetMyShortDescriptionParams {
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
