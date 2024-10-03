use serde::{Deserialize, Serialize};

use crate::entities::{
    revenue_withdrawal_state_failed::RevenueWithdrawalStateFailed,
    revenue_withdrawal_state_pending::RevenueWithdrawalStatePending,
    revenue_withdrawal_state_succeeded::RevenueWithdrawalStateSucceeded,
};

/// This object describes the state of a revenue withdrawal operation. Currently, it can be one of
///
/// * [RevenueWithdrawalStatePending](https://core.telegram.org/bots/api/#revenuewithdrawalstatepending)
/// * [RevenueWithdrawalStateSucceeded](https://core.telegram.org/bots/api/#revenuewithdrawalstatesucceeded)
/// * [RevenueWithdrawalStateFailed](https://core.telegram.org/bots/api/#revenuewithdrawalstatefailed)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#revenuewithdrawalstate)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RevenueWithdrawalState {
    /// The withdrawal is in progress.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#revenuewithdrawalstatepending)
    #[serde(rename = "pending")]
    Pending(RevenueWithdrawalStatePending),

    /// The withdrawal succeeded.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#revenuewithdrawalstatesucceeded)
    #[serde(rename = "succeeded")]
    Succeeded(RevenueWithdrawalStateSucceeded),

    /// The withdrawal failed and the transaction was refunded.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#revenuewithdrawalstatefailed)
    #[serde(rename = "failed")]
    Failed(RevenueWithdrawalStateFailed),
}

impl Default for RevenueWithdrawalState {
    fn default() -> Self {
        Self::Pending(RevenueWithdrawalStatePending::default())
    }
}

impl From<RevenueWithdrawalStatePending> for RevenueWithdrawalState {
    fn from(value: RevenueWithdrawalStatePending) -> Self {
        Self::Pending(value)
    }
}

impl From<RevenueWithdrawalStateSucceeded> for RevenueWithdrawalState {
    fn from(value: RevenueWithdrawalStateSucceeded) -> Self {
        Self::Succeeded(value)
    }
}

impl From<RevenueWithdrawalStateFailed> for RevenueWithdrawalState {
    fn from(value: RevenueWithdrawalStateFailed) -> Self {
        Self::Failed(value)
    }
}

// Divider: all content below this line will be preserved after code regen
