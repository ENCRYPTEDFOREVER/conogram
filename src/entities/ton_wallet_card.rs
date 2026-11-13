use serde::{Deserialize, Serialize};

use crate::entities::ton_wallet_card_sticker::TonWalletCardSticker;

/// This object describes a TON wallet card, which is shown on the profile page of a user to everyone.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#tonwalletcard)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TonWalletCard {
    /// Address of the wallet
    pub address: String,

    /// Balance of the wallet, in the *smallest units* of the cryptocurrency, as a decimal number
    pub amount: String,

    /// Stickers placed on the card
    pub stickers: Vec<TonWalletCardSticker>,

    /// *Optional*. Address of the NFT of one of the stickers, whose colors must be used for the card background; omitted if the default background must be used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_nft_address: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
