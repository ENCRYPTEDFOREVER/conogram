use serde::{Deserialize, Serialize};

use crate::entities::{area_position::AreaPosition, ton_nft::TonNft};

/// This object describes a sticker on a TON wallet card.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#tonwalletcardsticker)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TonWalletCardSticker {
    /// Position of the sticker on the card
    pub position: AreaPosition,

    /// The NFT shown as the sticker
    pub nft: TonNft,
}

// Divider: all content below this line will be preserved after code regen
