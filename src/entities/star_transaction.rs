use serde::{Deserialize, Serialize};

use crate::entities::transaction_partner::TransactionPartner;

/// Describes a Telegram Star transaction. Note that if the buyer initiates a chargeback with the payment provider from whom they acquired Stars (e.g., Apple, Google) following this transaction, the refunded Stars will be deducted from the bot's balance. This is outside of Telegram's control.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#startransaction)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StarTransaction {
    /// Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with *SuccessfulPayment.telegram\_payment\_charge\_id* for successful incoming payments from users.
    pub id: String,

    /// Integer amount of Telegram Stars transferred by the transaction
    pub amount: i64,

    /// *Optional*. The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i64>,

    /// Date the transaction was created in Unix time
    pub date: i64,

    /// *Optional*. Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<TransactionPartner>>,

    /// *Optional*. Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver: Option<Box<TransactionPartner>>,
}

// Divider: all content below this line will be preserved after code regen
