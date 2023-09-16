use crate::api::API;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct AnswerCallbackQueryParams {
    pub callback_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub show_alert: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
}

impl_into_future!(AnswerCallbackQueryRequest<'a>);

///Use this method to send answers to callback queries sent from [inline keyboards](https://core.telegram.org/bots/features#inline-keyboards). The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, *True* is returned.
///
///Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via [@BotFather](https://t.me/botfather) and accept the terms. Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
#[derive(Clone)]
pub struct AnswerCallbackQueryRequest<'a> {
    api: &'a API,
    params: AnswerCallbackQueryParams,
}

impl<'a> RequestT for AnswerCallbackQueryRequest<'a> {
    type ParamsType = AnswerCallbackQueryParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "answerCallbackQuery"
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
impl<'a> AnswerCallbackQueryRequest<'a> {
    pub fn new(api: &'a API, callback_query_id: String) -> Self {
        Self {
            api,
            params: AnswerCallbackQueryParams {
                callback_query_id,
                text: Option::default(),
                show_alert: bool::default(),
                url: Option::default(),
                cache_time: Option::default(),
            },
        }
    }

    ///Unique identifier for the query to be answered
    pub fn callback_query_id(mut self, callback_query_id: impl Into<String>) -> Self {
        self.params.callback_query_id = callback_query_id.into();
        self
    }

    ///Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.params.text = Some(text.into());
        self
    }

    ///If *True*, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to *false*.
    pub fn show_alert(mut self, show_alert: impl Into<bool>) -> Self {
        self.params.show_alert = show_alert.into();
        self
    }

    ///URL that will be opened by the user's client. If you have created a [Game](https://core.telegram.org/bots/api/#game) and accepted the conditions via [@BotFather](https://t.me/botfather), specify the URL that opens your game - note that this will only work if the query comes from a [*callback\_game*](https://core.telegram.org/bots/api/#inlinekeyboardbutton) button.  
    ///
    ///Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.params.url = Some(url.into());
        self
    }

    ///The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    pub fn cache_time(mut self, cache_time: impl Into<i64>) -> Self {
        self.params.cache_time = Some(cache_time.into());
        self
    }
}

impl<'a> API {
    ///Use this method to send answers to callback queries sent from [inline keyboards](https://core.telegram.org/bots/features#inline-keyboards). The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, *True* is returned.
    ///
    ///Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via [@BotFather](https://t.me/botfather) and accept the terms. Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
    pub fn answer_callback_query(
        &'a self,
        callback_query_id: impl Into<String>,
    ) -> AnswerCallbackQueryRequest {
        AnswerCallbackQueryRequest::new(self, callback_query_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
