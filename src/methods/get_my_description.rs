use conogram_derives::Request;
use serde::Serialize;

use crate::entities::bot_description::BotDescription;

/// Use this method to get the current bot description for the given user language. Returns [BotDescription](https://core.telegram.org/bots/api/#botdescription) on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getmydescription)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = BotDescription)]
pub struct GetMyDescriptionParams {
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
