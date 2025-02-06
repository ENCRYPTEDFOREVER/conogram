use conogram_derives::Request;
use serde::Serialize;

/// Verifies a user [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#verifyuser)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct VerifyUserParams {
    /// Unique identifier of the target user
    pub user_id: i64,

    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_description: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
