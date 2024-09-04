use crate::entities::animation::Animation;
use crate::entities::message_entity::MessageEntity;
use crate::entities::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

///This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
///
///API Reference: [link](https://core.telegram.org/bots/api/#game)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Game {
    ///Title of the game
    pub title: String,

    ///Description of the game
    pub description: String,

    ///Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,

    ///*Optional*. Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls [setGameScore](https://core.telegram.org/bots/api/#setgamescore), or manually edited using [editMessageText](https://core.telegram.org/bots/api/#editmessagetext). 0-4096 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    ///*Optional*. Special entities that appear in *text*, such as usernames, URLs, bot commands, etc.
    #[serde(default)]
    pub text_entities: Vec<MessageEntity>,

    ///*Optional*. Animation that will be displayed in the game message in chats. Upload via [BotFather](https://t.me/botfather)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}
// Divider: all content below this line will be preserved after code regen
