use serde::{Deserialize, Serialize};

///This object represents a service message about a forum topic closed in the chat. Currently holds no information.
///
///API Reference: [link](https://core.telegram.org/bots/api/#forumtopicclosed)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicClosed {}
// Divider: all content below this line will be preserved after code regen
