use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#deletechatphoto)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeleteChatPhotoParams {
    /// Unique identifier for the target chat or username of the target channel in the format `@username`
    pub chat_id: ChatId,
}

// Divider: all content below this line will be preserved after code regen
