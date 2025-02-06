use conogram_derives::Request;
use serde::Serialize;

use crate::entities::game_high_score::GameHighScore;

/// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of [GameHighScore](https://core.telegram.org/bots/api/#gamehighscore) objects.
///
/// This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and their neighbors are not among them. Please note that this behavior is subject to change.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getgamehighscores)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Vec<GameHighScore>)]
pub struct GetGameHighScoresParams {
    /// Target user id
    pub user_id: i64,

    /// Required if *inline\_message\_id* is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    /// Required if *inline\_message\_id* is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    /// Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
