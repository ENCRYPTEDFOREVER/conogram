use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{
    input_story_content::InputStoryContent, message_entity::MessageEntity, story::Story,
    story_area::StoryArea,
};

/// Edits a story previously posted by the bot on behalf of a managed business account. Requires the *can\_manage\_stories* business bot right. Returns [Story](https://core.telegram.org/bots/api/#story) on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#editstory)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Story)]
pub struct EditStoryParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// Unique identifier of the story to edit
    pub story_id: i64,

    /// Content of the story
    pub content: InputStoryContent,

    /// Caption of the story, 0-2048 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Mode for parsing entities in the story caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// A JSON-serialized list of clickable areas to be shown on the story
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub areas: Vec<StoryArea>,
}

// Divider: all content below this line will be preserved after code regen
