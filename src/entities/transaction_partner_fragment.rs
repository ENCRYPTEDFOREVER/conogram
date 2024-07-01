use crate::entities::revenue_withdrawal_state::RevenueWithdrawalState;
use serde::{Deserialize, Serialize};

///Describes a withdrawal transaction with Fragment.
///API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnerfragment)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerFragment {
    ///*Optional*. State of the transaction if the transaction is outgoing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}
// Divider: all content below this line will be preserved after code regen
