use serde::{Deserialize, Serialize};

use crate::entities::{ton_nft_attribute::TonNftAttribute, unique_gift::UniqueGift};

/// This object represents an NFT in the TON blockchain.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#tonnft)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TonNft {
    /// Address of the NFT
    pub address: String,

    /// *Optional*. Address of the collection to which the NFT belongs; omitted if unknown
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection_address: Option<String>,

    /// Index of the NFT among all NFTs from the collection as a decimal number
    pub index: String,

    /// Name of the NFT
    pub name: String,

    /// Description of the NFT
    pub description: String,

    /// *Optional*. Attributes of the NFT
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<TonNftAttribute>,

    /// *Optional*. The unique gift corresponding to the NFT
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gift: Option<UniqueGift>,
}

// Divider: all content below this line will be preserved after code regen
