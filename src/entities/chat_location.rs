use crate::entities::location::Location;
use serde::{Deserialize, Serialize};

///Represents a location to which a chat is connected.
///API Reference: [link](https://core.telegram.org/bots/api/#chatlocation)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatLocation {
    ///The location to which the supergroup is connected. Can't be a live location.
    pub location: Location,

    ///Location address; 1-64 characters, as defined by the chat owner
    pub address: String,
}
// Divider: all content below this line will be preserved after code regen
