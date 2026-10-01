use serde::{Deserialize, Serialize};

use crate::entities::community::Community;

/// Describes a service message about a chat being joined by a user from a community.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#communitychatjoined)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommunityChatJoined {
    /// The community from which the chat was joined
    pub community: Community,
}

// Divider: all content below this line will be preserved after code regen
