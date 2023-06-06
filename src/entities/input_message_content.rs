use crate::entities::input_contact_message_content::InputContactMessageContent;
use crate::entities::input_invoice_message_content::InputInvoiceMessageContent;
use crate::entities::input_location_message_content::InputLocationMessageContent;
use crate::entities::input_text_message_content::InputTextMessageContent;
use crate::entities::input_venue_message_content::InputVenueMessageContent;
use serde::Serialize;

///This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:
///
///* [InputTextMessageContent](https://core.telegram.org/bots/api/#inputtextmessagecontent)
///* [InputLocationMessageContent](https://core.telegram.org/bots/api/#inputlocationmessagecontent)
///* [InputVenueMessageContent](https://core.telegram.org/bots/api/#inputvenuemessagecontent)
///* [InputContactMessageContent](https://core.telegram.org/bots/api/#inputcontactmessagecontent)
///* [InputInvoiceMessageContent](https://core.telegram.org/bots/api/#inputinvoicemessagecontent)
///API Reference: [link](https://core.telegram.org/bots/api/#inputmessagecontent)
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    TextMessageContent(InputTextMessageContent),
    LocationMessageContent(InputLocationMessageContent),
    VenueMessageContent(InputVenueMessageContent),
    ContactMessageContent(InputContactMessageContent),
    InvoiceMessageContent(InputInvoiceMessageContent),
}
// Divider: all content below this line will be preserved after code regen
impl Default for InputMessageContent {
    fn default() -> Self {
        Self::TextMessageContent(InputTextMessageContent::default())
    }
}
