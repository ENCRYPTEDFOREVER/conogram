use crate::entities::chat_boost_source::ChatBoostSource;
use serde::{Deserialize, Serialize};

///This object contains information about a chat boost.
///
///API Reference: [link](https://core.telegram.org/bots/api/#chatboost)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatBoost {
    ///Unique identifier of the boost
    pub boost_id: String,

    ///Point in time (Unix timestamp) when the chat was boosted
    pub add_date: i64,

    ///Point in time (Unix timestamp) when the boost will automatically expire, unless the booster's Telegram Premium subscription is prolonged
    pub expiration_date: i64,

    ///Source of the added boost
    pub source: ChatBoostSource,
}
// Divider: all content below this line will be preserved after code regen
