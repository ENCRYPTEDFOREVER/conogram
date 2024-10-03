use serde::Serialize;

use crate::entities::web_app_info::WebAppInfo;

/// This object represents a button to be shown above inline query results. You **must** use exactly one of the optional fields.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultsbutton)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InlineQueryResultsButton {
    /// Label text on the button
    pub text: String,

    /// *Optional*. Description of the [Web App](https://core.telegram.org/bots/webapps) that will be launched when the user presses the button. The Web App will be able to switch back to the inline mode using the method [switchInlineQuery](https://core.telegram.org/bots/webapps#initializing-mini-apps) inside the Web App.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,

    /// *Optional*. [Deep-linking](https://core.telegram.org/bots/features#deep-linking) parameter for the /start message sent to the bot when a user presses the button. 1-64 characters, only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.  
    ///
    /// *Example:* An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a 'Connect your YouTube account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an OAuth link. Once done, the bot can offer a [*switch\_inline*](https://core.telegram.org/bots/api/#inlinekeyboardmarkup) button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
}

// Divider: all content below this line will be preserved after code regen

impl InlineQueryResultsButton {
    pub fn web_app(text: impl Into<String>, web_app: impl Into<WebAppInfo>) -> Self {
        Self {
            text: text.into(),
            web_app: Some(web_app.into()),
            start_parameter: None,
        }
    }

    pub fn start_parameter(text: impl Into<String>, start_parameter: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            web_app: None,
            start_parameter: Some(start_parameter.into()),
        }
    }
}
