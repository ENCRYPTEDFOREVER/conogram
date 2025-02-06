use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{chat_invite_link::ChatInviteLink, misc::chat_id::ChatId};

/// Use this method to create a [subscription invite link](https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions) for a channel chat. The bot must have the *can\_invite\_users* administrator rights. The link can be edited using the method [editChatSubscriptionInviteLink](https://core.telegram.org/bots/api/#editchatsubscriptioninvitelink) or revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#createchatsubscriptioninvitelink)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = ChatInviteLink)]
pub struct CreateChatSubscriptionInviteLinkParams {
    /// Unique identifier for the target channel chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The number of seconds the subscription will be active for before the next payment. Currently, it must always be 2592000 (30 days).
    pub subscription_period: i64,

    /// The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat; 1-2500
    pub subscription_price: i64,
}

// Divider: all content below this line will be preserved after code regen
