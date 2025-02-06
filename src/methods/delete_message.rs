use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to delete a message, including service messages, with the following limitations:  
/// \- A message can only be deleted if it was sent less than 48 hours ago.  
/// \- Service messages about a supergroup, channel, or forum topic creation can't be deleted.  
/// \- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.  
/// \- Bots can delete outgoing messages in private chats, groups, and supergroups.  
/// \- Bots can delete incoming messages in private chats.  
/// \- Bots granted *can\_post\_messages* permissions can delete outgoing messages in channels.  
/// \- If the bot is an administrator of a group, it can delete any message there.  
/// \- If the bot has *can\_delete\_messages* permission in a supergroup or a channel, it can delete any message there.  
/// Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#deletemessage)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeleteMessageParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Identifier of the message to delete
    pub message_id: i64,
}

// Divider: all content below this line will be preserved after code regen
