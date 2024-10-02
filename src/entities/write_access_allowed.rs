use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

/// This object represents a service message about a user allowing a bot to write messages after adding it to the attachment menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps).
///
/// API Reference: [link](https://core.telegram.org/bots/api/#writeaccessallowed)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WriteAccessAllowed {
    /// *Optional*. True, if the access was granted after the user accepted an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps)
    #[serde(default, skip_serializing_if = "is_false")]
    pub from_request: bool,

    /// *Optional*. Name of the Web App, if the access was granted when the Web App was launched from a link
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_app_name: Option<String>,

    /// *Optional*. True, if the access was granted when the bot was added to the attachment or side menu
    #[serde(default, skip_serializing_if = "is_false")]
    pub from_attachment_menu: bool,
}

// Divider: all content below this line will be preserved after code regen
