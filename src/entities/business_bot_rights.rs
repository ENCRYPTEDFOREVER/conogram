use serde::{Deserialize, Serialize};

use crate::utils::deserialize_utils::is_false;

/// Represents the rights of a business bot.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#businessbotrights)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessBotRights {
    /// *Optional*. True, if the bot can send and edit messages in the private chats that had incoming messages in the last 24 hours
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_reply: bool,

    /// *Optional*. True, if the bot can mark incoming private messages as read
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_read_messages: bool,

    /// *Optional*. True, if the bot can delete messages sent by the bot
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_delete_outgoing_messages: bool,

    /// *Optional*. True, if the bot can delete all private messages in managed chats
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_delete_all_messages: bool,

    /// *Optional*. True, if the bot can edit the first and last name of the business account
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_edit_name: bool,

    /// *Optional*. True, if the bot can edit the bio of the business account
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_edit_bio: bool,

    /// *Optional*. True, if the bot can edit the profile photo of the business account
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_edit_profile_photo: bool,

    /// *Optional*. True, if the bot can edit the username of the business account
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_edit_username: bool,

    /// *Optional*. True, if the bot can change the privacy settings pertaining to gifts for the business account
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_change_gift_settings: bool,

    /// *Optional*. True, if the bot can view gifts and the amount of Telegram Stars owned by the business account
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_view_gifts_and_stars: bool,

    /// *Optional*. True, if the bot can convert regular gifts owned by the business account to Telegram Stars
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_convert_gifts_to_stars: bool,

    /// *Optional*. True, if the bot can transfer and upgrade gifts owned by the business account
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_transfer_and_upgrade_gifts: bool,

    /// *Optional*. True, if the bot can transfer Telegram Stars received by the business account to its own account, or use them to upgrade and transfer gifts
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_transfer_stars: bool,

    /// *Optional*. True, if the bot can post, edit and delete stories on behalf of the business account
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_manage_stories: bool,
}

// Divider: all content below this line will be preserved after code regen
