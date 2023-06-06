use serde::{Deserialize, Serialize};

///This object represents a service message about General forum topic hidden in the chat. Currently holds no information.
///API Reference: [link](https://core.telegram.org/bots/api/#generalforumtopichidden)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GeneralForumTopicHidden {}
// Divider: all content below this line will be preserved after code regen
