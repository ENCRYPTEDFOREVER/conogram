use crate::entities::chat::Chat;
use crate::entities::chat_invite_link::ChatInviteLink;
use crate::entities::user::User;
use crate::utils::deserialize_utils::deserialize_boxed;
use serde::{Deserialize, Serialize};

///Represents a join request sent to a chat.
///API Reference: [link](https://core.telegram.org/bots/api/#chatjoinrequest)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatJoinRequest {
    ///Chat to which the request was sent
    #[serde(deserialize_with = "deserialize_boxed")]
    pub chat: Box<Chat>,

    ///User that sent the join request
    pub from: User,

    ///Identifier of a private chat with the user who sent the join request. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot can use this identifier for 24 hours to send messages until the join request is processed, assuming no other administrator contacted the user.
    pub user_chat_id: i64,

    ///Date the request was sent in Unix time
    pub date: i64,

    ///*Optional*. Bio of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    ///*Optional*. Chat invite link that was used by the user to send the join request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}
// Divider: all content below this line will be preserved after code regen
