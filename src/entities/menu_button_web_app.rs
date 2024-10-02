use crate::entities::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

/// Represents a menu button, which launches a [Web App](https://core.telegram.org/bots/webapps).
///
/// API Reference: [link](https://core.telegram.org/bots/api/#menubuttonwebapp)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonWebApp {
    /// Text on the button
    pub text: String,

    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [answerWebAppQuery](https://core.telegram.org/bots/api/#answerwebappquery). Alternatively, a `t.me` link to a Web App of the bot can be specified in the object instead of the Web App's URL, in which case the Web App will be opened as if the user pressed the link.
    pub web_app: WebAppInfo,
}

// Divider: all content below this line will be preserved after code regen
