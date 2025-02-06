use conogram_derives::Request;
use serde::Serialize;

use crate::entities::user::User;

/// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a [User](https://core.telegram.org/bots/api/#user) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getme)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = User)]
pub struct GetMeParams {}

// Divider: all content below this line will be preserved after code regen
