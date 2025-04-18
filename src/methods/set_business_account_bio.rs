use conogram_derives::Request;
use serde::Serialize;

/// Changes the bio of a managed business account. Requires the *can\_change\_bio* business bot right. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setbusinessaccountbio)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetBusinessAccountBioParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// The new value of the bio for the business account; 0-140 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
