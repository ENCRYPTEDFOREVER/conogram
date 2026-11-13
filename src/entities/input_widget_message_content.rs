use serde::Serialize;

use crate::{entities::message_entity::MessageEntity, utils::deserialize_utils::is_false};

/// Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a message with an [HTML widget](https://core.telegram.org/bots/widgets) to be sent as the result of an inline query. Currently, widgets can't be sent in response to inline queries in the production environment.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputwidgetmessagecontent)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputWidgetMessageContent {
    /// HTML code of the widget, 1-32768 bytes. The code must not exceed 4096 bytes when compressed by zlib with the maximum compression level.
    pub html: String,

    /// *Optional*. Width of the widget in logical pixels; omit if the widget must occupy the whole width of the message bubble. The width is clamped by the app to the range from 200 to the maximum width of the message bubble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// *Optional*. Height of the widget in logical pixels; omit if the height of the widget isn't fixed. The height is clamped by the app to the range 50-600.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// *Optional*. Public data of the widget, 0-1024 bytes, which will be available wherever the widget is shown. If the widget is shared, then all messages with it will have the same public data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_data: Option<String>,

    /// *Optional*. Data of the widget, which is specific to the message, 0-1024 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_data: Option<String>,

    /// *Optional*. Widget caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// *Optional*. Mode for parsing entities in the widget caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// *Optional*. Pass *True* if the caption must be shown above the widget
    #[serde(skip_serializing_if = "is_false")]
    pub show_caption_above_media: bool,

    /// *Optional*. Pass *True* to allow the widget to send queries to the bot
    #[serde(skip_serializing_if = "is_false")]
    pub allow_queries: bool,

    /// *Optional*. Pass *True* to create a shared widget. Each message sent from the result creates a separate shared widget, whose identifier is received by the bot only in the field *widget\_id* of queries from the widget. To send an existing shared widget in response to an inline query, use [InputSharedWidgetMessageContent](https://core.telegram.org/bots/api/#inputsharedwidgetmessagecontent) instead.
    #[serde(skip_serializing_if = "is_false")]
    pub is_shared: bool,
}

// Divider: all content below this line will be preserved after code regen
