use crate::entities::chat::Chat;
use crate::entities::chat_boost::ChatBoost;
use crate::utils::deserialize_utils::deserialize_boxed;
use serde::{Deserialize, Serialize};

///This object represents a boost added to a chat or changed.
///API Reference: [link](https://core.telegram.org/bots/api/#chatboostupdated)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostUpdated {
    ///Chat which was boosted
    #[serde(deserialize_with = "deserialize_boxed")]
    pub chat: Box<Chat>,

    ///Infomation about the chat boost
    pub boost: ChatBoost,
}
// Divider: all content below this line will be preserved after code regen
