use conogram_derives::Request;
use serde::Serialize;

/// Use this method to log out from the cloud Bot API server before launching the bot locally. You **must** log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns *True* on success. Requires no parameters.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#logout)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct LogOutParams {}

// Divider: all content below this line will be preserved after code regen
