use conogram_derives::Request;
use serde::Serialize;

/// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setmydescription)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetMyDescriptionParams {
    /// New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
