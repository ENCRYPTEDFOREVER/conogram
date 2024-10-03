use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that has no additional privileges or restrictions.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatmembermember)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberMember {
    /// Information about the user
    pub user: User,

    /// *Optional*. Date when the user's subscription will expire; Unix time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
