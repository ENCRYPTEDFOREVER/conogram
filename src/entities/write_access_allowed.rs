use serde::{Deserialize, Serialize};

///This object represents a service message about a user allowing a bot to write messages after adding the bot to the attachment menu or launching a Web App from a link.
///API Reference: [link](https://core.telegram.org/bots/api/#writeaccessallowed)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WriteAccessAllowed {
    ///*Optional*. Name of the Web App which was launched from a link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_name: Option<String>,
}
// Divider: all content below this line will be preserved after code regen
