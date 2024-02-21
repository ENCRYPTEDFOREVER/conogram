use crate::api::API;
use crate::entities::message::Message;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetGameScoreParams {
    pub user_id: i64,
    pub score: i64,
    #[serde(default, skip_serializing_if = "is_false")]
    pub force: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_edit_message: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

impl_into_future!(SetGameScoreRequest<'a>);

///Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Returns an error, if the new score is not greater than the user's current score in the chat and *force* is *False*.
#[derive(Clone)]
pub struct SetGameScoreRequest<'a> {
    api: &'a API,
    params: SetGameScoreParams,
}

impl<'a> RequestT for SetGameScoreRequest<'a> {
    type ParamsType = SetGameScoreParams;
    type ReturnType = Option<Message>;
    fn get_name() -> &'static str {
        "setGameScore"
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
impl<'a> SetGameScoreRequest<'a> {
    pub fn new(api: &'a API, user_id: impl Into<i64>, score: impl Into<i64>) -> Self {
        Self {
            api,
            params: SetGameScoreParams {
                user_id: user_id.into(),
                score: score.into(),
                force: bool::default(),
                disable_edit_message: bool::default(),
                chat_id: Option::default(),
                message_id: Option::default(),
                inline_message_id: Option::default(),
            },
        }
    }

    ///User identifier
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///New score, must be non-negative
    pub fn score(mut self, score: impl Into<i64>) -> Self {
        self.params.score = score.into();
        self
    }

    ///Pass *True* if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    pub fn force(mut self, force: impl Into<bool>) -> Self {
        self.params.force = force.into();
        self
    }

    ///Pass *True* if the game message should not be automatically edited to include the current scoreboard
    pub fn disable_edit_message(mut self, disable_edit_message: impl Into<bool>) -> Self {
        self.params.disable_edit_message = disable_edit_message.into();
        self
    }

    ///Required if *inline\_message\_id* is not specified. Unique identifier for the target chat
    pub fn chat_id(mut self, chat_id: impl Into<i64>) -> Self {
        self.params.chat_id = Some(chat_id.into());
        self
    }

    ///Required if *inline\_message\_id* is not specified. Identifier of the sent message
    pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
        self.params.message_id = Some(message_id.into());
        self
    }

    ///Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message
    pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(inline_message_id.into());
        self
    }
}

impl<'a> API {
    ///Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Returns an error, if the new score is not greater than the user's current score in the chat and *force* is *False*.
    pub fn set_game_score(
        &'a self,
        user_id: impl Into<i64>,
        score: impl Into<i64>,
    ) -> SetGameScoreRequest {
        SetGameScoreRequest::new(self, user_id, score)
    }
}

// Divider: all content below this line will be preserved after code regen
