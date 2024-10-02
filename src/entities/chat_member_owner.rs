use crate::entities::user::User;
use serde::{Deserialize, Serialize};

/// Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that owns the chat and has all administrator privileges.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatmemberowner)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberOwner {
    /// Information about the user
    pub user: User,

    /// *True*, if the user's presence in the chat is hidden
    pub is_anonymous: bool,

    /// *Optional*. Custom title for this user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
