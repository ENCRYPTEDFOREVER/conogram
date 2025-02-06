use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::{chat_id::ChatId, input_file::InputFile};

/// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setchatphoto)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetChatPhotoParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// New chat photo, uploaded using multipart/form-data
    pub photo: InputFile,
}

// Divider: all content below this line will be preserved after code regen
