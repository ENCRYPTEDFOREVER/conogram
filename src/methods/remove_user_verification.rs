use conogram_derives::Request;
use serde::Serialize;

/// Removes verification from a user who is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#removeuserverification)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct RemoveUserVerificationParams {
    /// Unique identifier of the target user
    pub user_id: i64,
}

// Divider: all content below this line will be preserved after code regen
