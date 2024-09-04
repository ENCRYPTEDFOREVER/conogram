use serde::{Deserialize, Serialize};

///Describes an inline message sent by a [Web App](https://core.telegram.org/bots/webapps) on behalf of a user.
///
///API Reference: [link](https://core.telegram.org/bots/api/#sentwebappmessage)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    ///*Optional*. Identifier of the sent inline message. Available only if there is an [inline keyboard](https://core.telegram.org/bots/api/#inlinekeyboardmarkup) attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
// Divider: all content below this line will be preserved after code regen
