use serde::{Deserialize, Serialize};

/// The reaction is paid.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#reactiontypepaid)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReactionTypePaid {}

// Divider: all content below this line will be preserved after code regen
