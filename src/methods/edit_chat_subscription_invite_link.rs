use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{chat_invite_link::ChatInviteLink, misc::chat_id::ChatId};

/// Use this method to edit a subscription invite link created by the bot. The bot must have the *can\_invite\_users* administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#editchatsubscriptioninvitelink)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = ChatInviteLink)]
pub struct EditChatSubscriptionInviteLinkParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// The invite link to edit
    pub invite_link: String,

    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
