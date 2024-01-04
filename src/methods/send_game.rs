use crate::api::API;
use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::message::Message;
use crate::entities::reply_parameters::ReplyParameters;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SendGameParams {
    pub chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub game_short_name: String,
    #[serde(skip_serializing_if = "is_false", default)]
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl_into_future!(SendGameRequest<'a>);

///Use this method to send a game. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
#[derive(Clone)]
pub struct SendGameRequest<'a> {
    api: &'a API,
    params: SendGameParams,
}

impl<'a> RequestT for SendGameRequest<'a> {
    type ParamsType = SendGameParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendGame"
    }
    fn get_api_ref(&self) -> &API {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        false
    }
}
impl<'a> SendGameRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<i64>, game_short_name: impl Into<String>) -> Self {
        Self {
            api,
            params: SendGameParams {
                chat_id: chat_id.into(),
                game_short_name: game_short_name.into(),
                message_thread_id: Option::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
                reply_parameters: Option::default(),
                reply_markup: Option::default(),
            },
        }
    }

    ///Unique identifier for the target chat
    pub fn chat_id(mut self, chat_id: impl Into<i64>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = Some(message_thread_id.into());
        self
    }

    ///Short name of the game, serves as the unique identifier for the game. Set up your games via [@BotFather](https://t.me/botfather).
    pub fn game_short_name(mut self, game_short_name: impl Into<String>) -> Self {
        self.params.game_short_name = game_short_name.into();
        self
    }

    ///Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
        self
    }

    ///Description of the message to reply to
    pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
        self.params.reply_parameters = Some(reply_parameters.into());
        self
    }

    ///A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If empty, one 'Play game\_title' button will be shown. If not empty, the first button must launch the game.
    pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl<'a> API {
    ///Use this method to send a game. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_game(
        &'a self,
        chat_id: impl Into<i64>,
        game_short_name: impl Into<String>,
    ) -> SendGameRequest {
        SendGameRequest::new(self, chat_id.into(), game_short_name.into())
    }
}

// Divider: all content below this line will be preserved after code regen
