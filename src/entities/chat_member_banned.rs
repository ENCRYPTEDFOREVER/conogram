use crate::entities::user::User;
use serde::{Deserialize, Serialize};

/// Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that was banned in the chat and can't return to the chat or view chat messages.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatmemberbanned)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberBanned {
    /// Information about the user
    pub user: User,

    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever
    pub until_date: i64,
}

// Divider: all content below this line will be preserved after code regen
