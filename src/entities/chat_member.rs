use crate::entities::chat_member_administrator::ChatMemberAdministrator;
use crate::entities::chat_member_banned::ChatMemberBanned;
use crate::entities::chat_member_left::ChatMemberLeft;
use crate::entities::chat_member_member::ChatMemberMember;
use crate::entities::chat_member_owner::ChatMemberOwner;
use crate::entities::chat_member_restricted::ChatMemberRestricted;
use serde::{Deserialize, Serialize};

///This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:
///
///* [ChatMemberOwner](https://core.telegram.org/bots/api/#chatmemberowner)
///* [ChatMemberAdministrator](https://core.telegram.org/bots/api/#chatmemberadministrator)
///* [ChatMemberMember](https://core.telegram.org/bots/api/#chatmembermember)
///* [ChatMemberRestricted](https://core.telegram.org/bots/api/#chatmemberrestricted)
///* [ChatMemberLeft](https://core.telegram.org/bots/api/#chatmemberleft)
///* [ChatMemberBanned](https://core.telegram.org/bots/api/#chatmemberbanned)
///API Reference: [link](https://core.telegram.org/bots/api/#chatmember)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ChatMember {
    #[serde(rename = "creator")]
    Owner(ChatMemberOwner),
    #[serde(rename = "administrator")]
    Administrator(ChatMemberAdministrator),
    #[serde(rename = "member")]
    Member(ChatMemberMember),
    #[serde(rename = "restricted")]
    Restricted(ChatMemberRestricted),
    #[serde(rename = "left")]
    Left(ChatMemberLeft),
    #[serde(rename = "kicked")]
    Banned(ChatMemberBanned),
}
// Divider: all content below this line will be preserved after code regen
impl Default for ChatMember {
    fn default() -> Self {
        Self::Left(ChatMemberLeft::default())
    }
}
