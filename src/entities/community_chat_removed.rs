use serde::{Deserialize, Serialize};

/// Describes a service message about a chat being removed from a community. Currently holds no information.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#communitychatremoved)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommunityChatRemoved {}

// Divider: all content below this line will be preserved after code regen
