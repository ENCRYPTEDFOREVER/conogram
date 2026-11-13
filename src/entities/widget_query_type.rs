use serde::{Deserialize, Serialize};

use crate::entities::{
    widget_query_type_data::WidgetQueryTypeData,
    widget_query_type_sent_transaction::WidgetQueryTypeSentTransaction,
    widget_query_type_sign_data::WidgetQueryTypeSignData,
};

/// This object describes the type of a query from a widget. Currently, it can be one of
///
/// * [WidgetQueryTypeData](https://core.telegram.org/bots/api/#widgetquerytypedata)
/// * [WidgetQueryTypeSignData](https://core.telegram.org/bots/api/#widgetquerytypesigndata)
/// * [WidgetQueryTypeSentTransaction](https://core.telegram.org/bots/api/#widgetquerytypesenttransaction)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#widgetquerytype)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WidgetQueryType {
    /// The query contains data for the bot.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#widgetquerytypedata)
    #[serde(rename = "data")]
    Data(WidgetQueryTypeData),

    /// The query contains data signed by a TON wallet of the user who sent the query. The data is signed for the domain `bot<bot_id>`, where `<bot_id>` is the identifier of the bot.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#widgetquerytypesigndata)
    #[serde(rename = "sign_data")]
    SignData(WidgetQueryTypeSignData),

    /// The query notifies the bot about a transaction sent from a TON wallet of the user who sent the query.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#widgetquerytypesenttransaction)
    #[serde(rename = "sent_transaction")]
    SentTransaction(WidgetQueryTypeSentTransaction),
}

impl Default for WidgetQueryType {
    fn default() -> Self {
        Self::Data(WidgetQueryTypeData::default())
    }
}

impl From<WidgetQueryTypeData> for WidgetQueryType {
    fn from(value: WidgetQueryTypeData) -> Self {
        Self::Data(value)
    }
}

impl From<WidgetQueryTypeSignData> for WidgetQueryType {
    fn from(value: WidgetQueryTypeSignData) -> Self {
        Self::SignData(value)
    }
}

impl From<WidgetQueryTypeSentTransaction> for WidgetQueryType {
    fn from(value: WidgetQueryTypeSentTransaction) -> Self {
        Self::SentTransaction(value)
    }
}

// Divider: all content below this line will be preserved after code regen
