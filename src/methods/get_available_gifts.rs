use conogram_derives::Request;
use serde::Serialize;

use crate::entities::gifts::Gifts;

/// Returns the list of gifts that can be sent by the bot to users and channel chats. Requires no parameters. Returns a [Gifts](https://core.telegram.org/bots/api/#gifts) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getavailablegifts)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Gifts)]
pub struct GetAvailableGiftsParams {}

// Divider: all content below this line will be preserved after code regen
