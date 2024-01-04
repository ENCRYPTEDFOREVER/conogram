use crate::entities::message::Message;
use serde::{Deserialize, Serialize};

///This object represents a service message about the completion of a giveaway without public winners.
///API Reference: [link](https://core.telegram.org/bots/api/#giveawaycompleted)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GiveawayCompleted {
    ///Number of winners in the giveaway
    pub winner_count: i64,

    ///*Optional*. Number of undistributed prizes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,

    ///*Optional*. Message with the giveaway that was completed, if it wasn't deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_message: Option<Box<Message>>,
}
// Divider: all content below this line will be preserved after code regen
