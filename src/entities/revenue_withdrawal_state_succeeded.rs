use serde::{Deserialize, Serialize};

///The withdrawal succeeded.
///
///API Reference: [link](https://core.telegram.org/bots/api/#revenuewithdrawalstatesucceeded)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RevenueWithdrawalStateSucceeded {
    ///Date the withdrawal was completed in Unix time
    pub date: i64,

    ///An HTTPS URL that can be used to see transaction details
    pub url: String,
}
// Divider: all content below this line will be preserved after code regen
