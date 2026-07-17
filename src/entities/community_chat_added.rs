use serde::{Deserialize, Serialize};

use crate::entities::community::Community;

/// Describes a service message about a chat being added to a community.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#communitychatadded)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommunityChatAdded {
    /// The new community to which the chat belongs
    pub community: Community,
}

// Divider: all content below this line will be preserved after code regen
