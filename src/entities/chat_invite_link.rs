use crate::entities::user::User;
use serde::{Deserialize, Serialize};

///Represents an invite link for a chat.
///
///API Reference: [link](https://core.telegram.org/bots/api/#chatinvitelink)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatInviteLink {
    ///The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with “…”.
    pub invite_link: String,

    ///Creator of the link
    pub creator: User,

    ///*True*, if users joining the chat via the link need to be approved by chat administrators
    pub creates_join_request: bool,

    ///*True*, if the link is primary
    pub is_primary: bool,

    ///*True*, if the link is revoked
    pub is_revoked: bool,

    ///*Optional*. Invite link name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    ///*Optional*. Point in time (Unix timestamp) when the link will expire or has been expired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,

    ///*Optional*. The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,

    ///*Optional*. Number of pending join requests created using this link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<i64>,

    ///*Optional*. The number of seconds the subscription will be active for before the next payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,

    ///*Optional*. The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat using the link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_price: Option<i64>,
}
// Divider: all content below this line will be preserved after code regen

use super::misc::chat_id::ChatId;
use crate::api::API;
use crate::methods::revoke_chat_invite_link::RevokeChatInviteLinkRequest;

impl ChatInviteLink {
    pub fn revoke<'a>(
        &'a self,
        api: &'a API,
        chat_id: impl Into<ChatId>,
    ) -> RevokeChatInviteLinkRequest {
        api.revoke_chat_invite_link(chat_id.into(), &self.invite_link)
    }
}
