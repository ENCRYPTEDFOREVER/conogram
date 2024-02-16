use crate::entities::chat::Chat;
use crate::utils::deserialize_utils::deserialize_boxed;
use serde::{Deserialize, Serialize};

///This object represents a story.
///API Reference: [link](https://core.telegram.org/bots/api/#story)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Story {
    ///Chat that posted the story
    #[serde(deserialize_with = "deserialize_boxed")]
    pub chat: Box<Chat>,

    ///Unique identifier for the story in the chat
    pub id: i64,
}
// Divider: all content below this line will be preserved after code regen
