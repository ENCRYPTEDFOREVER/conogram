use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// Describes a service message about an ownership change in the chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatownerchanged)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatOwnerChanged {
    /// The new owner of the chat
    pub new_owner: User,
}

// Divider: all content below this line will be preserved after code regen
