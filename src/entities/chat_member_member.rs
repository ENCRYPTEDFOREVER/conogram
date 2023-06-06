use crate::entities::user::User;
use serde::{Deserialize, Serialize};

///Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that has no additional privileges or restrictions.
///API Reference: [link](https://core.telegram.org/bots/api/#chatmembermember)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberMember {
    ///Information about the user
    pub user: User,
}
// Divider: all content below this line will be preserved after code regen
