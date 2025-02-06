use conogram_derives::Request;
use serde::Serialize;

/// Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method [requestEmojiStatusAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps). Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setuseremojistatus)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetUserEmojiStatusParams {
    /// Unique identifier of the target user
    pub user_id: i64,

    /// Custom emoji identifier of the emoji status to set. Pass an empty string to remove the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,

    /// Expiration date of the emoji status, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
