use crate::entities::chat_location::ChatLocation;
use crate::entities::chat_permissions::ChatPermissions;
use crate::entities::chat_photo::ChatPhoto;
use crate::entities::message::Message;
use crate::utils::deserialize_utils::deserialize_boxed_option;
use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///This object represents a chat.
///API Reference: [link](https://core.telegram.org/bots/api/#chat)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    ///Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,

    ///Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub type_: ChatType,

    ///*Optional*. Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    ///*Optional*. Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    ///*Optional*. First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    ///*Optional*. Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    ///*Optional*. *True*, if the supergroup chat is a forum (has [topics](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_forum: bool,

    ///*Optional*. Chat photo. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,

    ///*Optional*. If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames#collectible-usernames); for private chats, supergroups and channels. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(default)]
    pub active_usernames: Vec<String>,

    ///*Optional*. Custom emoji identifier of emoji status of the other party in a private chat. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,

    ///*Optional*. Bio of the other party in a private chat. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    ///*Optional*. *True*, if privacy settings of the other party in the private chat allows to use `tg://user?id=<user_id>` links only in chats with the user. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_private_forwards: bool,

    ///*Optional*. *True*, if the privacy settings of the other party restrict sending voice and video note messages in the private chat. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_restricted_voice_and_video_messages: bool,

    ///*Optional*. *True*, if users need to join the supergroup before they can send messages. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub join_to_send_messages: bool,

    ///*Optional*. *True*, if all users directly joining the supergroup need to be approved by supergroup administrators. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub join_by_request: bool,

    ///*Optional*. Description, for groups, supergroups and channel chats. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    ///*Optional*. Primary invite link, for groups, supergroups and channel chats. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,

    ///*Optional*. The most recent pinned message (by sending date). Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(
        deserialize_with = "deserialize_boxed_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub pinned_message: Option<Box<Message>>,

    ///*Optional*. Default chat member permissions, for groups and supergroups. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,

    ///*Optional*. For supergroups, the minimum allowed delay between consecutive messages sent by each unpriviledged user; in seconds. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i64>,

    ///*Optional*. The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i64>,

    ///*Optional*. *True*, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_aggressive_anti_spam_enabled: bool,

    ///*Optional*. *True*, if non-administrators can only get the list of bots and administrators in the chat. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_hidden_members: bool,

    ///*Optional*. *True*, if messages from the chat can't be forwarded to other chats. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_protected_content: bool,

    ///*Optional*. For supergroups, name of group sticker set. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,

    ///*Optional*. *True*, if the bot can change the group sticker set. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "is_false", default)]
    pub can_set_sticker_set: bool,

    ///*Optional*. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,

    ///*Optional*. For supergroups, the location to which the supergroup is connected. Returned only in [getChat](https://core.telegram.org/bots/api/#getchat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
}

///Type of chat, can be either “private”, “group”, “supergroup” or “channel”
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "type")]
pub enum ChatType {
    #[default]
    /// "private"
    #[serde(rename = "private")]
    Private,

    /// "group"
    #[serde(rename = "group")]
    Group,

    /// "supergroup"
    #[serde(rename = "supergroup")]
    Supergroup,

    /// "channel"
    #[serde(rename = "channel")]
    Channel,
}
// Divider: all content below this line will be preserved after code regen
