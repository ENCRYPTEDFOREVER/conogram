use serde::{Deserialize, Serialize};

/// Describes a service message about a transfer with a TON wallet.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#tonwallettransfer)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TonWalletTransfer {
    /// Unique identifier of the transaction; may be empty if the transfer hasn't been sent yet
    pub transaction_id: String,

    /// Address of the transfer peer in the TON blockchain
    pub peer_address: String,

    /// The amount of transferred cryptocurrency, in the *smallest units* of the cryptocurrency, as a decimal number; negative for outgoing transfers
    pub amount: String,

    /// *Optional*. Comment added to the transfer; omitted if the comment is encrypted
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
