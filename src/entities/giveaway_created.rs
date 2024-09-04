use serde::{Deserialize, Serialize};

///This object represents a service message about the creation of a scheduled giveaway. Currently holds no information.
///
///API Reference: [link](https://core.telegram.org/bots/api/#giveawaycreated)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GiveawayCreated {}
// Divider: all content below this line will be preserved after code regen
