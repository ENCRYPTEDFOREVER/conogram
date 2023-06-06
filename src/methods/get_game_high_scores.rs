use crate::api::API;
use crate::entities::game_high_score::GameHighScore;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct GetGameHighScoresParams {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

impl_into_future!(GetGameHighScoresRequest<'a>);

///Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of [GameHighScore](https://core.telegram.org/bots/api/#gamehighscore) objects.
///
///This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and their neighbors are not among them. Please note that this behavior is subject to change.
#[derive(Clone)]
pub struct GetGameHighScoresRequest<'a> {
    api: &'a API,
    params: GetGameHighScoresParams,
}

impl<'a> RequestT for GetGameHighScoresRequest<'a> {
    type ParamsType = GetGameHighScoresParams;
    type ReturnType = Vec<GameHighScore>;
    fn get_name() -> &'static str {
        "getGameHighScores"
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
impl<'a> GetGameHighScoresRequest<'a> {
    pub fn new(api: &'a API, user_id: i64) -> Self {
        Self {
            api,
            params: GetGameHighScoresParams {
                user_id,
                chat_id: Option::default(),
                message_id: Option::default(),
                inline_message_id: Option::default(),
            },
        }
    }

    ///Target user id
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
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
    ///Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of [GameHighScore](https://core.telegram.org/bots/api/#gamehighscore) objects.
    ///
    ///This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and their neighbors are not among them. Please note that this behavior is subject to change.
    pub fn get_game_high_scores(&'a self, user_id: impl Into<i64>) -> GetGameHighScoresRequest {
        GetGameHighScoresRequest::new(self, user_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
