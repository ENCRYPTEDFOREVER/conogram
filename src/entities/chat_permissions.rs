use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///Describes actions that a non-administrator user is allowed to take in a chat.
///API Reference: [link](https://core.telegram.org/bots/api/#chatpermissions)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatPermissions {
    ///*Optional*. *True*, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_send_messages: bool,

    ///*Optional*. *True*, if the user is allowed to send audios
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_send_audios: bool,

    ///*Optional*. *True*, if the user is allowed to send documents
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_send_documents: bool,

    ///*Optional*. *True*, if the user is allowed to send photos
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_send_photos: bool,

    ///*Optional*. *True*, if the user is allowed to send videos
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_send_videos: bool,

    ///*Optional*. *True*, if the user is allowed to send video notes
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_send_video_notes: bool,

    ///*Optional*. *True*, if the user is allowed to send voice notes
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_send_voice_notes: bool,

    ///*Optional*. *True*, if the user is allowed to send polls
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_send_polls: bool,

    ///*Optional*. *True*, if the user is allowed to send animations, games, stickers and use inline bots
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_send_other_messages: bool,

    ///*Optional*. *True*, if the user is allowed to add web page previews to their messages
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_add_web_page_previews: bool,

    ///*Optional*. *True*, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_change_info: bool,

    ///*Optional*. *True*, if the user is allowed to invite new users to the chat
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_invite_users: bool,

    ///*Optional*. *True*, if the user is allowed to pin messages. Ignored in public supergroups
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_pin_messages: bool,

    ///*Optional*. *True*, if the user is allowed to create forum topics. If omitted defaults to the value of can\_pin\_messages
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_manage_topics: bool,
}
// Divider: all content below this line will be preserved after code regen
impl ChatPermissions {
    /// An instance with **none** permissions
    pub fn none() -> Self {
        Self::default()
    }

    /// An instance with **all** permissions
    pub const fn all() -> Self {
        Self {
            can_send_messages: true,
            can_send_audios: true,
            can_send_documents: true,
            can_send_photos: true,
            can_send_videos: true,
            can_send_video_notes: true,
            can_send_voice_notes: true,
            can_send_polls: true,
            can_send_other_messages: true,
            can_add_web_page_previews: true,
            can_change_info: true,
            can_invite_users: true,
            can_pin_messages: true,
            can_manage_topics: true,
        }
    }
}

macro_rules! impl_chat_permissions_bitops {
    ($struct: ty => {$($trait: ident => $func: ident),*}, {$($trait_assign: ident => $func_assign: ident),*}) => {
        $(
            impl std::ops::$trait for $struct {
                type Output = Self;
                fn $func(self, rhs: Self) -> Self::Output {
                    Self {
                        can_send_messages: self.can_send_messages.$func(rhs.can_send_messages),
                        can_send_audios: self.can_send_audios.$func(rhs.can_send_audios),
                        can_send_documents: self.can_send_documents.$func(rhs.can_send_documents),
                        can_send_photos: self.can_send_photos.$func(rhs.can_send_photos),
                        can_send_videos: self.can_send_videos.$func(rhs.can_send_videos),
                        can_send_video_notes: self.can_send_video_notes.$func(rhs.can_send_video_notes),
                        can_send_voice_notes: self.can_send_voice_notes.$func(rhs.can_send_voice_notes),
                        can_send_polls: self.can_send_polls.$func(rhs.can_send_polls),
                        can_send_other_messages: self.can_send_other_messages.$func(rhs.can_send_other_messages),
                        can_add_web_page_previews: self.can_add_web_page_previews.$func(rhs.can_add_web_page_previews),
                        can_change_info: self.can_change_info.$func(rhs.can_change_info),
                        can_invite_users: self.can_invite_users.$func(rhs.can_invite_users),
                        can_pin_messages: self.can_pin_messages.$func(rhs.can_pin_messages),
                        can_manage_topics: self.can_manage_topics.$func(rhs.can_manage_topics),
                    }
                }
            }
        )*
        $(
            impl std::ops::$trait_assign for $struct {
                fn $func_assign(&mut self, rhs: Self) {
                    self.can_send_messages.$func_assign(rhs.can_send_messages);
                    self.can_send_audios.$func_assign(rhs.can_send_audios);
                    self.can_send_documents.$func_assign(rhs.can_send_documents);
                    self.can_send_photos.$func_assign(rhs.can_send_photos);
                    self.can_send_videos.$func_assign(rhs.can_send_videos);
                    self.can_send_video_notes.$func_assign(rhs.can_send_video_notes);
                    self.can_send_voice_notes.$func_assign(rhs.can_send_voice_notes);
                    self.can_send_polls.$func_assign(rhs.can_send_polls);
                    self.can_send_other_messages.$func_assign(rhs.can_send_other_messages);
                    self.can_add_web_page_previews.$func_assign(rhs.can_add_web_page_previews);
                    self.can_change_info.$func_assign(rhs.can_change_info);
                    self.can_invite_users.$func_assign(rhs.can_invite_users);
                    self.can_pin_messages.$func_assign(rhs.can_pin_messages);
                    self.can_manage_topics.$func_assign(rhs.can_manage_topics);
                }
            }
        )*
    };
}

impl_chat_permissions_bitops!(ChatPermissions =>
    {
        BitAnd => bitand,
        BitOr => bitor,
        BitXor => bitxor
    },
    {
        BitAndAssign => bitand_assign,
        BitOrAssign => bitor_assign,
        BitXorAssign => bitxor_assign
    }
);
