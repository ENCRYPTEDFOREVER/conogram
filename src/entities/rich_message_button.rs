use serde::{Deserialize, Serialize};

use crate::entities::{
    copy_text_button::CopyTextButton, disabled_button::DisabledButton, login_url::LoginUrl,
    rich_text::RichText, switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat,
    web_app_info::WebAppInfo,
};

/// This object represents a button in a [RichMessage](https://core.telegram.org/bots/api/#richmessage). Exactly one of the fields other than *text* and *style* must be used to specify the type of the button.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richmessagebutton)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichMessageButton {
    /// Text of the button. May contain only plain text, [RichTextCustomEmoji](https://core.telegram.org/bots/api/#richtextcustomemoji) and [RichTextDateTime](https://core.telegram.org/bots/api/#richtextdatetime) entities.
    pub text: Box<RichText>,

    /// *Optional*. Style of the button. Must be one of “danger”, “success”, “primary”, or “link” (the button is shown as a regular link without borders). Apps may use theme-specific colors for the button background and text based on the style. The style “link” is allowed only for callback buttons.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<RichMessageButtonStyle>,

    /// *Optional*. HTTP or tg:// URL to be opened when the button is pressed. Links `tg://user?id=<user_id>` can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// *Optional*. Data to be sent in a [callback query](https://core.telegram.org/bots/api/#callbackquery) to the bot when the button is pressed, 1-64 bytes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    /// *Optional*. Description of the [Web App](https://core.telegram.org/bots/webapps) that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [answerWebAppQuery](https://core.telegram.org/bots/api/#answerwebappquery). Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,

    /// *Optional*. An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the [Telegram Login Widget](https://core.telegram.org/widgets/login). Not supported for ephemeral messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,

    /// *Optional*. If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted. Not supported for messages sent in channel direct messages chats and on behalf of a business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,

    /// *Optional*. If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted. Not supported in channels and for messages sent in channel direct messages chats and on behalf of a business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,

    /// *Optional*. If set, pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field. Not supported for messages sent in channel direct messages chats and on behalf of a business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,

    /// *Optional*. A button that copies the specified text to the clipboard
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_text: Option<CopyTextButton>,

    /// *Optional*. If set, then the button is disabled and does nothing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<DisabledButton>,
}

/// *Optional*. Style of the button. Must be one of “danger”, “success”, “primary”, or “link” (the button is shown as a regular link without borders). Apps may use theme-specific colors for the button background and text based on the style. The style “link” is allowed only for callback buttons.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RichMessageButtonStyle {
    /// `danger`
    #[default]
    #[serde(rename = "danger")]
    Danger,

    /// `success`
    #[serde(rename = "success")]
    Success,

    /// `primary`
    #[serde(rename = "primary")]
    Primary,

    /// `link`
    #[serde(rename = "link")]
    Link,
}

// Divider: all content below this line will be preserved after code regen

impl RichMessageButton {
    /// HTTP or tg:// URL to be opened when the button is pressed. Links `tg://user?id=<user_id>` can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.
    pub fn url(
        text: impl Into<RichText>,
        url: impl Into<String>,
        style: Option<RichMessageButtonStyle>,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            url: Some(url.into()),
            style,
            ..Default::default()
        }
    }

    /// Data to be sent in a [callback query](https://core.telegram.org/bots/api/#callbackquery) to the bot when the button is pressed, 1-64 bytes
    pub fn callback(
        text: impl Into<RichText>,
        data: impl Into<String>,
        style: Option<RichMessageButtonStyle>,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            callback_data: Some(data.into()),
            style,
            ..Default::default()
        }
    }

    /// [Web App](https://core.telegram.org/bots/webapps) that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [answerWebAppQuery](https://core.telegram.org/bots/api/#answerwebappquery). Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a business account.
    ///
    /// An HTTPS URL of a Web App to be opened with additional data as specified in [Initializing Web Apps](https://core.telegram.org/bots/webapps#initializing-mini-apps)
    pub fn web_app(
        text: impl Into<RichText>,
        url: impl Into<String>,
        style: Option<RichMessageButtonStyle>,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            web_app: Some(WebAppInfo { url: url.into() }),
            style,
            ..Default::default()
        }
    }

    /// An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the [Telegram Login Widget](https://core.telegram.org/widgets/login). Not supported for ephemeral messages.
    pub fn login(
        text: impl Into<RichText>,
        url: impl Into<LoginUrl>,
        style: Option<RichMessageButtonStyle>,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            login_url: Some(url.into()),
            style,
            ..Default::default()
        }
    }

    /// Pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted. Not supported for messages sent in channel direct messages chats and on behalf of a business account.
    pub fn switch_inline_query(
        text: impl Into<RichText>,
        query: impl Into<String>,
        style: Option<RichMessageButtonStyle>,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            switch_inline_query: Some(query.into()),
            style,
            ..Default::default()
        }
    }

    /// Pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted. Not supported in channels and for messages sent in channel direct messages chats and on behalf of a business account.
    pub fn switch_inline_query_current_chat(
        text: impl Into<RichText>,
        query: impl Into<String>,
        style: Option<RichMessageButtonStyle>,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            switch_inline_query_current_chat: Some(query.into()),
            style,
            ..Default::default()
        }
    }

    /// Pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field. Not supported for messages sent in channel direct messages chats and on behalf of a business account.
    pub fn switch_inline_query_chosen_chat(
        text: impl Into<RichText>,
        query: impl Into<SwitchInlineQueryChosenChat>,
        style: Option<RichMessageButtonStyle>,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            switch_inline_query_chosen_chat: Some(query.into()),
            style,
            ..Default::default()
        }
    }

    /// A button that copies the specified text to the clipboard, 1-256 characters
    pub fn copy_text(
        text: impl Into<RichText>,
        copy_text: impl Into<String>,
        style: Option<RichMessageButtonStyle>,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            copy_text: Some(CopyTextButton {
                text: copy_text.into(),
            }),
            style,
            ..Default::default()
        }
    }

    /// Disabled button which does nothing
    pub fn disabled(text: impl Into<RichText>, style: Option<RichMessageButtonStyle>) -> Self {
        Self {
            text: Box::new(text.into()),
            disabled: Some(DisabledButton {}),
            style,
            ..Default::default()
        }
    }
}
