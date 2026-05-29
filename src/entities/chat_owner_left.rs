use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// Describes a service message about the chat owner leaving the chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatownerleft)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatOwnerLeft {
    /// *Optional*. The user who will become the new owner of the chat if the previous owner does not return to the chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_owner: Option<User>,
}

// Divider: all content below this line will be preserved after code regen
