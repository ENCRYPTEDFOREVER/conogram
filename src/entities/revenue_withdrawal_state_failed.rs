use serde::{Deserialize, Serialize};

///The withdrawal failed and the transaction was refunded.
///API Reference: [link](https://core.telegram.org/bots/api/#revenuewithdrawalstatefailed)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RevenueWithdrawalStateFailed {}
// Divider: all content below this line will be preserved after code regen
