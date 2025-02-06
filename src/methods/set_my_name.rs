use conogram_derives::Request;
use serde::Serialize;

/// Use this method to change the bot's name. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setmyname)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetMyNameParams {
    /// New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
