use serde::{Deserialize, Serialize};

use crate::{
    entities::{
        accepted_gift_types::AcceptedGiftTypes,
        birthdate::Birthdate,
        business_intro::BusinessIntro,
        business_location::BusinessLocation,
        business_opening_hours::BusinessOpeningHours,
        chat::{Chat, ChatType},
        chat_location::ChatLocation,
        chat_permissions::ChatPermissions,
        chat_photo::ChatPhoto,
        message::Message,
        reaction_type::ReactionType,
    },
    utils::deserialize_utils::is_false,
};

/// This object contains full information about a chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatfullinfo)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatFullInfo {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,

    /// Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub type_: ChatType,

    /// *Optional*. Title, for supergroups, channels and group chats
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// *Optional*. Username, for private chats, supergroups and channels if available
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// *Optional*. First name of the other party in a private chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// *Optional*. Last name of the other party in a private chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// *Optional*. *True*, if the supergroup chat is a forum (has [topics](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_forum: bool,

    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See [accent colors](https://core.telegram.org/bots/api/#accent-colors) for more details.
    pub accent_color_id: i64,

    /// The maximum number of reactions that can be set on a message in the chat
    pub max_reaction_count: i64,

    /// *Optional*. Chat photo
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,

    /// *Optional*. If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames#collectible-usernames); for private chats, supergroups and channels
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active_usernames: Vec<String>,

    /// *Optional*. For private chats, the date of birth of the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<Birthdate>,

    /// *Optional*. For private chats with business accounts, the intro of the business
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_intro: Option<BusinessIntro>,

    /// *Optional*. For private chats with business accounts, the location of the business
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_location: Option<BusinessLocation>,

    /// *Optional*. For private chats with business accounts, the opening hours of the business
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_opening_hours: Option<BusinessOpeningHours>,

    /// *Optional*. For private chats, the personal channel of the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_chat: Option<Box<Chat>>,

    /// *Optional*. List of available reactions allowed in the chat. If omitted, then all [emoji reactions](https://core.telegram.org/bots/api/#reactiontypeemoji) are allowed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_reactions: Vec<ReactionType>,

    /// *Optional*. Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_custom_emoji_id: Option<String>,

    /// *Optional*. Identifier of the accent color for the chat's profile background. See [profile accent colors](https://core.telegram.org/bots/api/#profile-accent-colors) for more details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_accent_color_id: Option<i64>,

    /// *Optional*. Custom emoji identifier of the emoji chosen by the chat for its profile background
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_background_custom_emoji_id: Option<String>,

    /// *Optional*. Custom emoji identifier of the emoji status of the chat or the other party in a private chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,

    /// *Optional*. Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,

    /// *Optional*. Bio of the other party in a private chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    /// *Optional*. *True*, if privacy settings of the other party in the private chat allows to use `tg://user?id=<user_id>` links only in chats with the user
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_private_forwards: bool,

    /// *Optional*. *True*, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_restricted_voice_and_video_messages: bool,

    /// *Optional*. *True*, if users need to join the supergroup before they can send messages
    #[serde(default, skip_serializing_if = "is_false")]
    pub join_to_send_messages: bool,

    /// *Optional*. *True*, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
    #[serde(default, skip_serializing_if = "is_false")]
    pub join_by_request: bool,

    /// *Optional*. Description, for groups, supergroups and channel chats
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// *Optional*. Primary invite link, for groups, supergroups and channel chats
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,

    /// *Optional*. The most recent pinned message (by sending date)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,

    /// *Optional*. Default chat member permissions, for groups and supergroups
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,

    /// Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
    pub accepted_gift_types: AcceptedGiftTypes,

    /// *Optional*. *True*, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_send_paid_media: bool,

    /// *Optional*. For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i64>,

    /// *Optional*. For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<i64>,

    /// *Optional*. The time after which all messages sent to the chat will be automatically deleted; in seconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i64>,

    /// *Optional*. *True*, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_aggressive_anti_spam_enabled: bool,

    /// *Optional*. *True*, if non-administrators can only get the list of bots and administrators in the chat
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_hidden_members: bool,

    /// *Optional*. *True*, if messages from the chat can't be forwarded to other chats
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_protected_content: bool,

    /// *Optional*. *True*, if new chat members will have access to old messages; available only to chat administrators
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_visible_history: bool,

    /// *Optional*. For supergroups, name of the group sticker set
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,

    /// *Optional*. *True*, if the bot can change the group sticker set
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_set_sticker_set: bool,

    /// *Optional*. For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_emoji_sticker_set_name: Option<String>,

    /// *Optional*. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,

    /// *Optional*. For supergroups, the location to which the supergroup is connected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
}

// Divider: all content below this line will be preserved after code regen
