use conogram_derives::Request;
use serde::Serialize;

/// Changes the username of a managed business account. Requires the *can\_change\_username* business bot right. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setbusinessaccountusername)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetBusinessAccountUsernameParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// The new value of the username for the business account; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
