use conogram_derives::Request;
use serde::Serialize;

/// Use this method to revoke the current token of a managed bot and generate a new one. Returns the new token as *String* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#replacemanagedbottoken)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = String)]
pub struct ReplaceManagedBotTokenParams {
    /// User identifier of the managed bot whose token will be replaced
    pub user_id: i64,
}

// Divider: all content below this line will be preserved after code regen
