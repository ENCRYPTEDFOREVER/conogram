use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// This object represents the content of a service message, sent whenever a user in the chat triggers a proximity alert set by another user.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#proximityalerttriggered)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert
    pub traveler: User,

    /// User that set the alert
    pub watcher: User,

    /// The distance between the users
    pub distance: i64,
}

// Divider: all content below this line will be preserved after code regen
