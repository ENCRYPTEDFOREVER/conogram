use crate::entities::star_transaction::StarTransaction;
use serde::{Deserialize, Serialize};

///Contains a list of Telegram Star transactions.
///API Reference: [link](https://core.telegram.org/bots/api/#startransactions)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StarTransactions {
    ///The list of transactions
    pub transactions: Vec<StarTransaction>,
}
// Divider: all content below this line will be preserved after code regen
