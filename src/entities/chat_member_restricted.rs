use crate::entities::user::User;
use serde::{Deserialize, Serialize};

///Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that is under certain restrictions in the chat. Supergroups only.
///API Reference: [link](https://core.telegram.org/bots/api/#chatmemberrestricted)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberRestricted {
    ///Information about the user
    pub user: User,

    ///*True*, if the user is a member of the chat at the moment of the request
    pub is_member: bool,

    ///*True*, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    pub can_send_messages: bool,

    ///*True*, if the user is allowed to send audios
    pub can_send_audios: bool,

    ///*True*, if the user is allowed to send documents
    pub can_send_documents: bool,

    ///*True*, if the user is allowed to send photos
    pub can_send_photos: bool,

    ///*True*, if the user is allowed to send videos
    pub can_send_videos: bool,

    ///*True*, if the user is allowed to send video notes
    pub can_send_video_notes: bool,

    ///*True*, if the user is allowed to send voice notes
    pub can_send_voice_notes: bool,

    ///*True*, if the user is allowed to send polls
    pub can_send_polls: bool,

    ///*True*, if the user is allowed to send animations, games, stickers and use inline bots
    pub can_send_other_messages: bool,

    ///*True*, if the user is allowed to add web page previews to their messages
    pub can_add_web_page_previews: bool,

    ///*True*, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,

    ///*True*, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,

    ///*True*, if the user is allowed to pin messages
    pub can_pin_messages: bool,

    ///*True*, if the user is allowed to create forum topics
    pub can_manage_topics: bool,

    ///Date when restrictions will be lifted for this user; Unix time. If 0, then the user is restricted forever
    pub until_date: i64,
}
// Divider: all content below this line will be preserved after code regen
use super::chat_permissions::ChatPermissions;

impl ChatMemberRestricted {
    pub fn permissions(&self) -> ChatPermissions {
        ChatPermissions {
            can_send_messages: self.can_send_messages,
            can_send_audios: self.can_send_audios,
            can_send_documents: self.can_send_documents,
            can_send_photos: self.can_send_photos,
            can_send_videos: self.can_send_videos,
            can_send_video_notes: self.can_send_video_notes,
            can_send_voice_notes: self.can_send_voice_notes,
            can_send_polls: self.can_send_polls,
            can_send_other_messages: self.can_send_other_messages,
            can_add_web_page_previews: self.can_add_web_page_previews,
            can_change_info: self.can_change_info,
            can_invite_users: self.can_invite_users,
            can_pin_messages: self.can_pin_messages,
            can_manage_topics: self.can_manage_topics,
        }
    }
}
