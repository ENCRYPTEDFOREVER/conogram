use crate::entities::message_origin_channel::MessageOriginChannel;
use crate::entities::message_origin_chat::MessageOriginChat;
use crate::entities::message_origin_hidden_user::MessageOriginHiddenUser;
use crate::entities::message_origin_user::MessageOriginUser;
use serde::{Deserialize, Serialize};

///This object describes the origin of a message. It can be one of
///
///* [MessageOriginUser](https://core.telegram.org/bots/api/#messageoriginuser)
///* [MessageOriginHiddenUser](https://core.telegram.org/bots/api/#messageoriginhiddenuser)
///* [MessageOriginChat](https://core.telegram.org/bots/api/#messageoriginchat)
///* [MessageOriginChannel](https://core.telegram.org/bots/api/#messageoriginchannel)
///
///API Reference: [link](https://core.telegram.org/bots/api/#messageorigin)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageOrigin {
    #[serde(rename = "user")]
    User(MessageOriginUser),
    #[serde(rename = "hidden_user")]
    HiddenUser(MessageOriginHiddenUser),
    #[serde(rename = "chat")]
    Chat(MessageOriginChat),
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
