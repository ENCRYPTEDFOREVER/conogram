use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to edit data of a widget in a message sent by the bot or via the bot (for inline bots). Public data of a shared widget can be changed at most once per second. On success, *True* is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#editmessagewidgetdata)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct EditMessageWidgetDataParams {
    /// Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target supergroup in the format `@username`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    /// Required if *inline\_message\_id* is not specified. Identifier of the message with the widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    /// Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    /// New public data of the widget, 0-1024 bytes; pass an empty string to remove the public data. If omitted, then the public data isn't changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_data: Option<String>,

    /// New data of the widget, which is specific to the message, 0-1024 bytes; pass an empty string to remove the data. If omitted, then the data isn't changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_data: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
