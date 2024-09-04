use serde::{Deserialize, Serialize};

///This object represents a service message about an edited forum topic.
///
///API Reference: [link](https://core.telegram.org/bots/api/#forumtopicedited)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicEdited {
    ///*Optional*. New name of the topic, if it was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    ///*Optional*. New identifier of the custom emoji shown as the topic icon, if it was edited; an empty string if the icon was removed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}
// Divider: all content below this line will be preserved after code regen
