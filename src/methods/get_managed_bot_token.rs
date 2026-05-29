use conogram_derives::Request;
use serde::Serialize;

/// Use this method to get the token of a managed bot. Returns the token as *String* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getmanagedbottoken)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = String)]
pub struct GetManagedBotTokenParams {
    /// User identifier of the managed bot whose token will be returned
    pub user_id: i64,
}

// Divider: all content below this line will be preserved after code regen
