use serde::Serialize;

/// Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a contact message to be sent as the result of an inline query.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputcontactmessagecontent)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,

    /// Contact's first name
    pub first_name: String,

    /// *Optional*. Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// *Optional*. Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
