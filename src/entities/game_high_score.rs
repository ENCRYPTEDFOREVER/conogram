use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// This object represents one row of the high scores table for a game.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#gamehighscore)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: i64,

    /// User
    pub user: User,

    /// Score
    pub score: i64,
}

// Divider: all content below this line will be preserved after code regen
