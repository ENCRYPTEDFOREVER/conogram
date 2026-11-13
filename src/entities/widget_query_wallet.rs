use serde::{Deserialize, Serialize};

/// This object describes a TON wallet of the user, which is used in a query from a widget.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#widgetquerywallet)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidgetQueryWallet {
    /// Address of the wallet
    pub address: String,

    /// Balance of the wallet, in the *smallest units* of the cryptocurrency, as a decimal number
    pub amount: String,

    /// *Optional*. Hex-encoded public key of the wallet; omitted if unknown
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,

    /// *Optional*. Base64-encoded serialized StateInit of the wallet contract; omitted if and only if *public\_key* is omitted
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_init: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
