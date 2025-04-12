use conogram_derives::Request;
use serde::Serialize;

/// Changes the first and last name of a managed business account. Requires the *can\_change\_name* business bot right. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setbusinessaccountname)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetBusinessAccountNameParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// The new value of the first name for the business account; 1-64 characters
    pub first_name: String,

    /// The new value of the last name for the business account; 0-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
