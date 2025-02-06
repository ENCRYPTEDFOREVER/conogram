use conogram_derives::Request;
use serde::Serialize;

use crate::utils::deserialize_utils::is_false;

/// Use this method to send answers to callback queries sent from [inline keyboards](https://core.telegram.org/bots/features#inline-keyboards). The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, *True* is returned.
///
/// Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via [@BotFather](https://t.me/botfather) and accept the terms. Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#answercallbackquery)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct AnswerCallbackQueryParams {
    /// Unique identifier for the query to be answered
    pub callback_query_id: String,

    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// If *True*, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to *false*.
    #[serde(skip_serializing_if = "is_false")]
    pub show_alert: bool,

    /// URL that will be opened by the user's client. If you have created a [Game](https://core.telegram.org/bots/api/#game) and accepted the conditions via [@BotFather](https://t.me/botfather), specify the URL that opens your game - note that this will only work if the query comes from a [*callback\_game*](https://core.telegram.org/bots/api/#inlinekeyboardbutton) button.  
    ///
    /// Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
