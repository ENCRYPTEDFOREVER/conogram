use serde::{Deserialize, Serialize};

use crate::entities::{
    chat::Chat, inline_query::ChatType, user::User, widget_query_type::WidgetQueryType,
};

/// This object represents an incoming query from a widget in a message sent by the bot or via the bot. Use [answerWidgetQuery](https://core.telegram.org/bots/api/#answerwidgetquery) to answer the query. Each user can send at most 5 queries per 5 seconds to the bot from all its widgets.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#widgetquery)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WidgetQuery {
    /// Unique identifier for this query
    pub id: String,

    /// Sender
    pub from: User,

    /// *Optional*. Chat with the message containing the widget; omitted if the widget is in an inline message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat: Option<Box<Chat>>,

    /// *Optional*. Identifier of the message containing the widget; omitted if the widget is in an inline message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    /// *Optional*. Identifier of the inline message containing the widget
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    /// Global identifier, uniquely corresponding to the chat to which the message with the widget was sent
    pub chat_instance: String,

    /// *Optional*. For widgets in inline messages, type of the chat to which the message was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<ChatType>,

    /// Type of the query
    #[serde(rename = "type")]
    pub type_: WidgetQueryType,

    /// Data of the query, 1-64 bytes
    pub data: String,

    /// *Optional*. Unique identifier of the shared widget, which sent the query; omitted if the widget isn't shared
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_id: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
