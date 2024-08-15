use crate::entities::revenue_withdrawal_state_failed::RevenueWithdrawalStateFailed;
use crate::entities::revenue_withdrawal_state_pending::RevenueWithdrawalStatePending;
use crate::entities::revenue_withdrawal_state_succeeded::RevenueWithdrawalStateSucceeded;
use serde::{Deserialize, Serialize};

///This object describes the state of a revenue withdrawal operation. Currently, it can be one of
///
///* [RevenueWithdrawalStatePending](https://core.telegram.org/bots/api/#revenuewithdrawalstatepending)
///* [RevenueWithdrawalStateSucceeded](https://core.telegram.org/bots/api/#revenuewithdrawalstatesucceeded)
///* [RevenueWithdrawalStateFailed](https://core.telegram.org/bots/api/#revenuewithdrawalstatefailed)
///
///API Reference: [link](https://core.telegram.org/bots/api/#revenuewithdrawalstate)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RevenueWithdrawalState {
    #[serde(rename = "pending")]
    Pending(RevenueWithdrawalStatePending),
    #[serde(rename = "succeeded")]
    Succeeded(RevenueWithdrawalStateSucceeded),
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
