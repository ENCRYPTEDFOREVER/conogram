use serde::{Deserialize, Serialize};

///This object represents a service message about a forum topic reopened in the chat. Currently holds no information.
///API Reference: [link](https://core.telegram.org/bots/api/#forumtopicreopened)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicReopened {}
// Divider: all content below this line will be preserved after code regen
