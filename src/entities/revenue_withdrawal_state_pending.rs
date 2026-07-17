use serde::{Deserialize, Serialize};

/// The withdrawal is in progress.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#revenuewithdrawalstatepending)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "pending", tag = "type")]
pub struct RevenueWithdrawalStatePending {}

// Divider: all content below this line will be preserved after code regen
