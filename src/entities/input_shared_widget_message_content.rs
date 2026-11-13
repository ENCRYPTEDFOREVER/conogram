use serde::Serialize;

use crate::{entities::message_entity::MessageEntity, utils::deserialize_utils::is_false};

/// Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a message with a shared widget to be sent as the result of an inline query. Currently, widgets can't be sent in response to inline queries in the production environment.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputsharedwidgetmessagecontent)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputSharedWidgetMessageContent {
    /// Unique identifier of the shared widget created by the bot
    pub widget_id: String,

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
}

// Divider: all content below this line will be preserved after code regen
