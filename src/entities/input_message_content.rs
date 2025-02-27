use serde::Serialize;

use crate::entities::{
    input_contact_message_content::InputContactMessageContent,
    input_invoice_message_content::InputInvoiceMessageContent,
    input_location_message_content::InputLocationMessageContent,
    input_text_message_content::InputTextMessageContent,
    input_venue_message_content::InputVenueMessageContent,
};

/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:
///
/// * [InputTextMessageContent](https://core.telegram.org/bots/api/#inputtextmessagecontent)
/// * [InputLocationMessageContent](https://core.telegram.org/bots/api/#inputlocationmessagecontent)
/// * [InputVenueMessageContent](https://core.telegram.org/bots/api/#inputvenuemessagecontent)
/// * [InputContactMessageContent](https://core.telegram.org/bots/api/#inputcontactmessagecontent)
/// * [InputInvoiceMessageContent](https://core.telegram.org/bots/api/#inputinvoicemessagecontent)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputmessagecontent)
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    /// Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a text message to be sent as the result of an inline query.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputtextmessagecontent)
    TextMessageContent(InputTextMessageContent),

    /// Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a location message to be sent as the result of an inline query.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputlocationmessagecontent)
    LocationMessageContent(InputLocationMessageContent),

    /// Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a venue message to be sent as the result of an inline query.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputvenuemessagecontent)
    VenueMessageContent(InputVenueMessageContent),

    /// Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a contact message to be sent as the result of an inline query.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputcontactmessagecontent)
    ContactMessageContent(InputContactMessageContent),

    /// Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of an invoice message to be sent as the result of an inline query.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputinvoicemessagecontent)
    InvoiceMessageContent(InputInvoiceMessageContent),
}

impl Default for InputMessageContent {
    fn default() -> Self {
        Self::TextMessageContent(InputTextMessageContent::default())
    }
}

impl From<InputTextMessageContent> for InputMessageContent {
    fn from(value: InputTextMessageContent) -> Self {
        Self::TextMessageContent(value)
    }
}

impl From<InputLocationMessageContent> for InputMessageContent {
    fn from(value: InputLocationMessageContent) -> Self {
        Self::LocationMessageContent(value)
    }
}

impl From<InputVenueMessageContent> for InputMessageContent {
    fn from(value: InputVenueMessageContent) -> Self {
        Self::VenueMessageContent(value)
    }
}

impl From<InputContactMessageContent> for InputMessageContent {
    fn from(value: InputContactMessageContent) -> Self {
        Self::ContactMessageContent(value)
    }
}

impl From<InputInvoiceMessageContent> for InputMessageContent {
    fn from(value: InputInvoiceMessageContent) -> Self {
        Self::InvoiceMessageContent(value)
    }
}

// Divider: all content below this line will be preserved after code regen
