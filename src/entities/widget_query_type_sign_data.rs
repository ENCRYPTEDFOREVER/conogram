use serde::{Deserialize, Serialize};

use crate::entities::widget_query_wallet::WidgetQueryWallet;

/// The query contains data signed by a TON wallet of the user who sent the query. The data is signed for the domain `bot<bot_id>`, where `<bot_id>` is the identifier of the bot.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#widgetquerytypesigndata)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "sign_data", tag = "type")]
pub struct WidgetQueryTypeSignData {
    /// The wallet that signed the data
    pub wallet: WidgetQueryWallet,

    /// JSON-serialized signed data
    pub sign_data: String,
}

// Divider: all content below this line will be preserved after code regen
