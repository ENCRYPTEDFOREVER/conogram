use crate::entities::user::User;
use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///The boost was obtained by the creation of a Telegram Premium giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
///API Reference: [link](https://core.telegram.org/bots/api/#chatboostsourcegiveaway)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostSourceGiveaway {
    ///Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.
    pub giveaway_message_id: i64,

    ///*Optional*. User that won the prize in the giveaway if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    ///*Optional*. True, if the giveaway was completed, but there was no user to win the prize
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_unclaimed: bool,
}
// Divider: all content below this line will be preserved after code regen
