use serde::{Deserialize, Serialize};

use crate::entities::{
    message_origin_channel::MessageOriginChannel, message_origin_chat::MessageOriginChat,
    message_origin_hidden_user::MessageOriginHiddenUser, message_origin_user::MessageOriginUser,
};

/// This object describes the origin of a message. It can be one of
///
/// * [MessageOriginUser](https://core.telegram.org/bots/api/#messageoriginuser)
/// * [MessageOriginHiddenUser](https://core.telegram.org/bots/api/#messageoriginhiddenuser)
/// * [MessageOriginChat](https://core.telegram.org/bots/api/#messageoriginchat)
/// * [MessageOriginChannel](https://core.telegram.org/bots/api/#messageoriginchannel)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#messageorigin)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageOrigin {
    /// The message was originally sent by a known user.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#messageoriginuser)
    #[serde(rename = "user")]
    User(MessageOriginUser),

    /// The message was originally sent by an unknown user.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#messageoriginhiddenuser)
    #[serde(rename = "hidden_user")]
    HiddenUser(MessageOriginHiddenUser),

    /// The message was originally sent on behalf of a chat to a group chat.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#messageoriginchat)
    #[serde(rename = "chat")]
    Chat(MessageOriginChat),

    /// The message was originally sent to a channel chat.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#messageoriginchannel)
    #[serde(rename = "channel")]
    Channel(MessageOriginChannel),
}

impl Default for MessageOrigin {
    fn default() -> Self {
        Self::User(MessageOriginUser::default())
    }
}

impl From<MessageOriginUser> for MessageOrigin {
    fn from(value: MessageOriginUser) -> Self {
        Self::User(value)
    }
}

impl From<MessageOriginHiddenUser> for MessageOrigin {
    fn from(value: MessageOriginHiddenUser) -> Self {
        Self::HiddenUser(value)
    }
}

impl From<MessageOriginChat> for MessageOrigin {
    fn from(value: MessageOriginChat) -> Self {
        Self::Chat(value)
    }
}

impl From<MessageOriginChannel> for MessageOrigin {
    fn from(value: MessageOriginChannel) -> Self {
        Self::Channel(value)
    }
}

// Divider: all content below this line will be preserved after code regen
