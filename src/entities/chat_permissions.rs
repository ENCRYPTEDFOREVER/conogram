use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///Describes actions that a non-administrator user is allowed to take in a chat.
///API Reference: [link](https://core.telegram.org/bots/api/#chatpermissions)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatPermissions {
    ///*Optional*. *True*, if the user is allowed to send text messages, contacts, invoices, locations and venues
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_send_messages: bool,

    ///*Optional*. *True*, if the user is allowed to send audios
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_send_audios: bool,

    ///*Optional*. *True*, if the user is allowed to send documents
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_send_documents: bool,

    ///*Optional*. *True*, if the user is allowed to send photos
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_send_photos: bool,

    ///*Optional*. *True*, if the user is allowed to send videos
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_send_videos: bool,

    ///*Optional*. *True*, if the user is allowed to send video notes
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_send_video_notes: bool,

    ///*Optional*. *True*, if the user is allowed to send voice notes
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_send_voice_notes: bool,

    ///*Optional*. *True*, if the user is allowed to send polls
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_send_polls: bool,

    ///*Optional*. *True*, if the user is allowed to send animations, games, stickers and use inline bots
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_send_other_messages: bool,

    ///*Optional*. *True*, if the user is allowed to add web page previews to their messages
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_add_web_page_previews: bool,

    ///*Optional*. *True*, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_change_info: bool,

    ///*Optional*. *True*, if the user is allowed to invite new users to the chat
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_invite_users: bool,

    ///*Optional*. *True*, if the user is allowed to pin messages. Ignored in public supergroups
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_pin_messages: bool,

    ///*Optional*. *True*, if the user is allowed to create forum topics. If omitted defaults to the value of can\_pin\_messages
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_manage_topics: bool,
}
// Divider: all content below this line will be preserved after code regen
