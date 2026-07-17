use serde::Serialize;

use crate::entities::input_rich_message::InputRichMessage;

/// Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a rich message to be sent as the result of an inline query.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichmessagecontent)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputRichMessageContent {
    /// The message to be sent
    pub rich_message: InputRichMessage,
}

// Divider: all content below this line will be preserved after code regen
