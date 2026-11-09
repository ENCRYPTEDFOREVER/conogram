use hmac::{Hmac, KeyInit, Mac};
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use sha2::Sha256;

use crate::{
    entities::{chat::ChatType, inline_query::ChatType as InlineChatType},
    utils::{ToHex, deserialize_utils::is_false},
};

fn deserialize_json_string<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => serde_json::from_str(&s)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

/// This object contains data that is transferred to the Mini App when it is opened. It is empty if the Mini App was launched from a [keyboard button](https://core.telegram.org/bots/webapps#keyboard-button-mini-apps) or from [inline mode](https://core.telegram.org/bots/webapps#inline-mode-mini-apps).
///
/// API Reference: [link](https://core.telegram.org/bots/webapps#webappinitdata)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebAppInitData {
    /// *Optional*. A unique identifier for the Mini App session, required for sending messages via the [answerWebAppQuery](https://core.telegram.org/bots/api#answerwebappquery) method.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,

    /// *Optional*. A unique identifier for the chat join request query, required for processing the query via the method [answerChatJoinRequestQuery](https://core.telegram.org/bots/api#answerchatjoinrequestquery).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_join_request_query_id: Option<String>,

    /// *Optional*. An object containing data about the current user.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_json_string"
    )]
    pub user: Option<WebAppUser>,

    /// *Optional*. An object containing data about the chat partner of the current user in the chat where the bot was launched via the attachment menu. Returned only for private chats and only for Mini Apps launched via the attachment menu.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_json_string"
    )]
    pub receiver: Option<WebAppUser>,

    /// *Optional*. An object containing data about the chat where the bot was launched via the attachment menu or the chat to which the join request query was sent. Returned for supergroups, channels and group chats – only for Mini Apps launched via the attachment menu and chat join requests.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_json_string"
    )]
    pub chat: Option<WebAppChat>,

    /// *Optional*. Type of the chat from which the Mini App was opened. Can be either “sender” for a private chat with the user opening the link, “private”, “group”, “supergroup”, or “channel”. Returned only for Mini Apps launched from direct links.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<InlineChatType>,

    /// *Optional*. Global identifier, uniquely corresponding to the chat from which the Mini App was opened. Returned only for Mini Apps launched from a direct link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_instance: Option<String>,

    /// *Optional*. The value of the *startattach* parameter, passed [via link](https://core.telegram.org/bots/webapps#adding-bots-to-the-attachment-menu). Only returned for Mini Apps when launched from the attachment menu via link.
    ///
    /// The value of the `start_param` parameter will also be passed in the GET-parameter `tgWebAppStartParam`, so the Mini App can load the correct interface right away.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_param: Option<String>,

    /// *Optional*. Time in seconds, after which a message can be sent via the [answerWebAppQuery](https://core.telegram.org/bots/api#answerwebappquery) method.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_send_after: Option<i64>,

    /// Unix time when the form was opened.
    pub auth_date: i64,

    /// A hash of all passed parameters, which the bot server can use to [check their validity](https://core.telegram.org/bots/webapps#validating-data-received-via-the-mini-app).
    pub hash: String,

    /// A signature of all passed parameters (except *hash*), which the third party can use to [check their validity](https://core.telegram.org/bots/webapps#validating-data-for-third-party-use).
    pub signature: String,
}

impl WebAppInitData {
    /// Validate provided urlencoded web app init data string.
    ///
    /// See: [validating](https://core.telegram.org/bots/webapps#validating-data-received-via-the-mini-app)
    pub fn validate_hash(init_data_str: impl AsRef<str>, bot_token: impl AsRef<str>) -> bool {
        let (init_data_str, bot_token) = (init_data_str.as_ref(), bot_token.as_ref());

        let params: std::collections::BTreeMap<String, String> =
            form_urlencoded::parse(init_data_str.as_bytes())
                .into_owned()
                .collect();

        let Some(received_hash) = params.get("hash") else {
            return false;
        };

        let data_check_string: String = params
            .iter()
            .filter(|(k, _)| *k != "hash")
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("\n");

        let Ok(mut secret_mac) = Hmac::<Sha256>::new_from_slice(b"WebAppData") else {
            return false;
        };
        secret_mac.update(bot_token.as_bytes());
        let secret_key = secret_mac.finalize().into_bytes();

        let Ok(mut calc_mac) = Hmac::<Sha256>::new_from_slice(&secret_key) else {
            return false;
        };
        calc_mac.update(data_check_string.as_bytes());
        let computed_hash = calc_mac.finalize().into_bytes().hex();

        computed_hash == *received_hash
    }

    pub fn from_urlenc_str(
        init_data_str: impl AsRef<str>,
    ) -> Result<Self, serde_urlencoded::de::Error> {
        serde_urlencoded::from_str(init_data_str.as_ref())
    }
}

/// This object contains the data of the Mini App user.
///
/// API Reference: [link](https://core.telegram.org/bots/webapps#webappuser)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebAppUser {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,

    /// *Optional*. True, if this user is a bot. Returns in the [receiver](https://core.telegram.org/bots/webapps#webappinitdata) field only.
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_bot: bool,

    /// First name of the user or bot.
    pub first_name: String,

    /// *Optional*. Last name of the user or bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// *Optional*. Username of the user or bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// *Optional*. [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language. Returns in user field only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// *Optional*. *True*, if this user is a Telegram Premium user.
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_premium: bool,

    /// *Optional*. *True*, if this user added the bot to the attachment menu.
    #[serde(default, skip_serializing_if = "is_false")]
    pub added_to_attachment_menu: bool,

    /// *Optional*. *True*, if this user allowed the bot to message them.
    #[serde(default, skip_serializing_if = "is_false")]
    pub allows_write_to_pm: bool,

    /// *Optional*. URL of the user’s profile photo. The photo can be in .jpeg or .svg formats.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
}

/// This object represents a chat.
///
/// API Reference: [link](https://core.telegram.org/bots/webapps#webappchat)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebAppChat {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,

    /// Type of chat, can be either “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub type_: ChatType,

    /// Title of the chat
    pub title: String,

    /// *Optional*. Username of the chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// *Optional*. URL of the chat’s photo. The photo can be in .jpeg or .svg formats. Only returned for Mini Apps launched from the attachment menu.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
}

/// This object describes the native popup.
///
/// API Reference: [link](https://core.telegram.org/bots/webapps#popupparams)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PopupParams {
    /// *Optional*. The text to be displayed in the popup title, 0-64 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// The message to be displayed in the body of the popup, 1-256 characters.
    pub message: String,

    /// *Optional*. List of buttons to be displayed in the popup, 1-3 buttons. Set to [{“type”:“close”}] by default.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub buttons: Vec<PopupButton>,
}

/// This object describes the native popup button.
///
/// API Reference: [link](https://core.telegram.org/bots/webapps#popupbutton)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PopupButton {
    /// *Optional*. Identifier of the button, 0-64 characters. Set to empty string by default.
    ///
    /// If the button is pressed, its *id* is returned in the callback and the *popupClosed* event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// *Optional*. Type of the button. Set to default by default.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<PopupButtonType>,

    /// *Optional*. The text to be displayed on the button, 0-64 characters. Required if *type* is *default* or *destructive*. Irrelevant for other types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl PopupButton {
    /// Buttons with text, 0-64 characters
    ///
    /// If the button is pressed, its *id* is returned in the callback and the *popupClosed* event.
    pub fn text(text: impl Into<String>, id: Option<String>) -> Self {
        Self {
            id,
            type_: Some(PopupButtonType::Default),
            text: Some(text.into()),
        }
    }

    /// A button with a style that indicates a destructive action (e.g. “Remove”, “Delete”, etc.)
    ///
    /// If the button is pressed, its *id* is returned in the callback and the *popupClosed* event.
    pub fn destructive(text: impl Into<String>, id: Option<String>) -> Self {
        Self {
            id,
            type_: Some(PopupButtonType::Destructive),
            text: Some(text.into()),
        }
    }

    /// A button with the localized text “OK”
    ///
    /// If the button is pressed, its *id* is returned in the callback and the *popupClosed* event.
    #[must_use]
    pub const fn ok(id: Option<String>) -> Self {
        Self {
            id,
            type_: Some(PopupButtonType::Ok),
            text: None,
        }
    }

    /// A button with the localized text “Close”
    ///
    /// If the button is pressed, its *id* is returned in the callback and the *popupClosed* event.
    #[must_use]
    pub const fn close(id: Option<String>) -> Self {
        Self {
            id,
            type_: Some(PopupButtonType::Close),
            text: None,
        }
    }

    /// A button with the localized text “Cancel”
    ///
    /// If the button is pressed, its *id* is returned in the callback and the *popupClosed* event.
    #[must_use]
    pub const fn cancel(id: Option<String>) -> Self {
        Self {
            id,
            type_: Some(PopupButtonType::Cancel),
            text: None,
        }
    }
}

/// Type of the button
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PopupButtonType {
    /// A button with the default style
    #[default]
    #[serde(rename = "default")]
    Default,
    /// A button with the localized text “OK”
    #[serde(rename = "ok")]
    Ok,
    /// A button with the localized text “Close”
    #[serde(rename = "close")]
    Close,
    /// A button with the localized text “Cancel”
    #[serde(rename = "cancel")]
    Cancel,
    /// A button with a style that indicates a destructive action (e.g. “Remove”, “Delete”, etc.)
    #[serde(rename = "destructive")]
    Destructive,
}
