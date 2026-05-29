use conogram_derives::Request;
use serde::Serialize;

use crate::entities::star_amount::StarAmount;

/// A method to get the current Telegram Stars balance of the bot. Requires no parameters. On success, returns a [StarAmount](https://core.telegram.org/bots/api/#staramount) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getmystarbalance)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = StarAmount)]
pub struct GetMyStarBalanceParams {}

// Divider: all content below this line will be preserved after code regen
