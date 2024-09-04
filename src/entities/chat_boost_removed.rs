use crate::entities::chat::Chat;
use crate::entities::chat_boost_source::ChatBoostSource;
use crate::utils::deserialize_utils::deserialize_boxed;
use serde::{Deserialize, Serialize};

///This object represents a boost removed from a chat.
///
///API Reference: [link](https://core.telegram.org/bots/api/#chatboostremoved)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostRemoved {
    ///Chat which was boosted
    #[serde(deserialize_with = "deserialize_boxed")]
    pub chat: Box<Chat>,

    ///Unique identifier of the boost
    pub boost_id: String,

    ///Point in time (Unix timestamp) when the boost was removed
    pub remove_date: i64,

    ///Source of the removed boost
    pub source: ChatBoostSource,
}
// Divider: all content below this line will be preserved after code regen
