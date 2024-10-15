use serde::{Deserialize, Serialize};

use crate::entities::{inaccessible_message::InaccessibleMessage, message::Message};

/// This object describes a message that can be inaccessible to the bot. It can be one of
///
/// * [Message](https://core.telegram.org/bots/api/#message)
/// * [InaccessibleMessage](https://core.telegram.org/bots/api/#inaccessiblemessage)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#maybeinaccessiblemessage)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    /// This object represents a message.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#message)
    Message(Message),

    /// This object describes a message that was deleted or is otherwise inaccessible to the bot.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inaccessiblemessage)
    InaccessibleMessage(InaccessibleMessage),
}

impl Default for MaybeInaccessibleMessage {
    fn default() -> Self {
        Self::Message(Message::default())
    }
}

impl From<Message> for MaybeInaccessibleMessage {
    fn from(value: Message) -> Self {
        Self::Message(value)
    }
}

impl From<InaccessibleMessage> for MaybeInaccessibleMessage {
    fn from(value: InaccessibleMessage) -> Self {
        Self::InaccessibleMessage(value)
    }
}

// Divider: all content below this line will be preserved after code regen
