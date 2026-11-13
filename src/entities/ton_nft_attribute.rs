use serde::{Deserialize, Serialize};

/// This object represents an attribute of an NFT in the TON blockchain.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#tonnftattribute)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TonNftAttribute {
    /// Trait type of the attribute
    pub trait_type: String,

    /// Value of the attribute
    pub value: String,
}

// Divider: all content below this line will be preserved after code regen
