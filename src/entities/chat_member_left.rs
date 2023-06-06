use crate::entities::user::User;
use serde::{Deserialize, Serialize};

///Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that isn't currently a member of the chat, but may join it themselves.
///API Reference: [link](https://core.telegram.org/bots/api/#chatmemberleft)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberLeft {
    ///Information about the user
    pub user: User,
}
// Divider: all content below this line will be preserved after code regen
