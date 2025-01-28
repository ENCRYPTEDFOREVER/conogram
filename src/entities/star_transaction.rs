use serde::{Deserialize, Serialize};

use crate::entities::transaction_partner::TransactionPartner;

/// Describes a Telegram Star transaction.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#startransaction)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarTransaction {
    /// Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with *SuccessfulPayment.telegram\_payment\_charge\_id* for successful incoming payments from users.
    pub id: String,

    /// Number of Telegram Stars transferred by the transaction
    pub amount: i64,

    /// Date the transaction was created in Unix time
    pub date: i64,

    /// *Optional*. Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<TransactionPartner>,

    /// *Optional*. Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver: Option<TransactionPartner>,
}

// Divider: all content below this line will be preserved after code regen
