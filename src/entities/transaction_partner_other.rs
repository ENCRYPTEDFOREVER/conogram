use serde::{Deserialize, Serialize};

///Describes a transaction with an unknown source or recipient.
///API Reference: [link](https://core.telegram.org/bots/api/#transactionpartnerother)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionPartnerOther {}
// Divider: all content below this line will be preserved after code regen
