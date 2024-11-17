use serde::{Deserialize, Serialize};

use crate::entities::gift::Gift;

/// This object represent a list of gifts.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#gifts)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Gifts {
    /// The list of gifts
    pub gifts: Vec<Gift>,
}

// Divider: all content below this line will be preserved after code regen
