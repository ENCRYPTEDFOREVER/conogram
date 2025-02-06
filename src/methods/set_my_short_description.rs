use conogram_derives::Request;
use serde::Serialize;

/// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setmyshortdescription)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetMyShortDescriptionParams {
    /// New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    /// A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
