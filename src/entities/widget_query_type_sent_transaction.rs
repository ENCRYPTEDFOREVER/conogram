use serde::{Deserialize, Serialize};

use crate::entities::widget_query_wallet::WidgetQueryWallet;

/// The query notifies the bot about a transaction sent from a TON wallet of the user who sent the query.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#widgetquerytypesenttransaction)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "sent_transaction", tag = "type")]
pub struct WidgetQueryTypeSentTransaction {
    /// The wallet from which the transaction was sent
    pub wallet: WidgetQueryWallet,

    /// Value of *msg\_hash* of the transaction
    pub msg_hash: String,
}

// Divider: all content below this line will be preserved after code regen
