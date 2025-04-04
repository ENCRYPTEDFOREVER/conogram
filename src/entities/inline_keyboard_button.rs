use serde::{Deserialize, Serialize};

use crate::{
    entities::{
        callback_game::CallbackGame, copy_text_button::CopyTextButton, login_url::LoginUrl,
        switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat, web_app_info::WebAppInfo,
    },
    utils::deserialize_utils::is_false,
};

/// This object represents one button of an inline keyboard. Exactly one of the optional fields must be used to specify type of the button.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inlinekeyboardbutton)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,

    /// *Optional*. HTTP or tg:// URL to be opened when the button is pressed. Links `tg://user?id=<user_id>` can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// *Optional*. Data to be sent in a [callback query](https://core.telegram.org/bots/api/#callbackquery) to the bot when the button is pressed, 1-64 bytes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    /// *Optional*. Description of the [Web App](https://core.telegram.org/bots/webapps) that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [answerWebAppQuery](https://core.telegram.org/bots/api/#answerwebappquery). Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a Telegram Business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,

    /// *Optional*. An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the [Telegram Login Widget](https://core.telegram.org/widgets/login).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,

    /// *Optional*. If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted. Not supported for messages sent on behalf of a Telegram Business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,

    /// *Optional*. If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted.  
    ///
    /// This offers a quick way for the user to open your bot in inline mode in the same chat - good for selecting something from multiple options. Not supported in channels and for messages sent on behalf of a Telegram Business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,

    /// *Optional*. If set, pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field. Not supported for messages sent on behalf of a Telegram Business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,

    /// *Optional*. Description of the button that copies the specified text to the clipboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_text: Option<CopyTextButton>,

    /// *Optional*. Description of the game that will be launched when the user presses the button.  
    ///
    /// **NOTE:** This type of button **must** always be the first button in the first row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,

    /// *Optional*. Specify *True*, to send a [Pay button](https://core.telegram.org/bots/api/#payments). Substrings “⭐” and “XTR” in the buttons's text will be replaced with a Telegram Star icon.  
    ///
    /// **NOTE:** This type of button **must** always be the first button in the first row and can only be used in invoice messages.
    #[serde(default, skip_serializing_if = "is_false")]
    pub pay: bool,
}

// Divider: all content below this line will be preserved after code regen

impl InlineKeyboardButton {
    pub fn switch_inline_query_current_chat(
        text: impl Into<String>,
        query: impl Into<String>,
    ) -> Self {
        Self {
            text: text.into(),
            switch_inline_query_current_chat: Some(query.into()),
            ..Default::default()
        }
    }

    pub fn switch_inline_query_chosen_chat(
        text: impl Into<String>,
        query: impl Into<SwitchInlineQueryChosenChat>,
    ) -> Self {
        Self {
            text: text.into(),
            switch_inline_query_chosen_chat: Some(query.into()),
            ..Default::default()
        }
    }

    pub fn switch_inline_query(text: impl Into<String>, query: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            switch_inline_query: Some(query.into()),
            ..Default::default()
        }
    }

    pub fn callback(text: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            callback_data: Some(data.into()),
            ..Default::default()
        }
    }

    pub fn copy_text(text: impl Into<String>, copy_text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            copy_text: Some(CopyTextButton {
                text: copy_text.into(),
            }),
            ..Default::default()
        }
    }

    pub fn web_app(text: impl Into<String>, web_app_url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            web_app: Some(WebAppInfo {
                url: web_app_url.into(),
            }),
            ..Default::default()
        }
    }
}
