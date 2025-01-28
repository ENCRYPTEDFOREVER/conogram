use serde::{Deserialize, Serialize};

/// This object represents a phone contact.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#contact)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,

    /// Contact's first name
    pub first_name: String,

    /// *Optional*. Contact's last name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// *Optional*. Contact's user identifier in Telegram. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// *Optional*. Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
