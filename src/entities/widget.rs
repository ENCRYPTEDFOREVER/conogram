use serde::{Deserialize, Serialize};

/// This object describes an [HTML widget](https://core.telegram.org/bots/widgets).
///
/// API Reference: [link](https://core.telegram.org/bots/api/#widget)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Widget {
    /// *Optional*. Unique identifier of the shared widget, which can be used to send the widget again using the method [sendSharedWidget](https://core.telegram.org/bots/api/#sendsharedwidget); omitted if the widget isn't shared
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// *Optional*. Width of the widget in logical pixels; omitted if the widget must occupy the whole width of the message bubble
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// *Optional*. Height of the widget in logical pixels; omitted if the height of the widget isn't fixed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// HTML code of the widget
    pub html: String,

    /// *Optional*. Public data of the widget, which is available wherever the widget is shown. All messages with a shared widget have the same public data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_data: Option<String>,

    /// *Optional*. Data of the widget, which is specific to the message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_data: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
