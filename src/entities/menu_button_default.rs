use serde::{Deserialize, Serialize};

///Describes that no specific value for the menu button was set.
///
///API Reference: [link](https://core.telegram.org/bots/api/#menubuttondefault)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonDefault {}
// Divider: all content below this line will be preserved after code regen
