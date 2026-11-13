use serde::{Deserialize, Serialize};

use crate::entities::ton_wallet_card::TonWalletCard;

/// The query contains data for the bot.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#widgetquerytypedata)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "data", tag = "type")]
pub struct WidgetQueryTypeData {
    /// *Optional*. TON wallet card of the user who sent the query, if it was sent to the bot along with the query
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wallet_card: Option<TonWalletCard>,
}

// Divider: all content below this line will be preserved after code regen
