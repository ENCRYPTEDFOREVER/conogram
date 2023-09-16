use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see [ReplyKeyboardMarkup](https://core.telegram.org/bots/api/#replykeyboardmarkup)).
///API Reference: [link](https://core.telegram.org/bots/api/#replykeyboardremove)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    ///Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use *one\_time\_keyboard* in [ReplyKeyboardMarkup](https://core.telegram.org/bots/api/#replykeyboardmarkup))
    pub remove_keyboard: bool,

    ///*Optional*. Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the *text* of the [Message](https://core.telegram.org/bots/api/#message) object; 2) if the bot's message is a reply (has *reply\_to\_message\_id*), sender of the original message.  
    ///
    ///*Example:* A user votes in a poll, bot returns confirmation message in reply to the vote and removes the keyboard for that user, while still showing the keyboard with poll options to users who haven't voted yet.
    #[serde(default, skip_serializing_if = "is_false")]
    pub selective: bool,
}
// Divider: all content below this line will be preserved after code regen
