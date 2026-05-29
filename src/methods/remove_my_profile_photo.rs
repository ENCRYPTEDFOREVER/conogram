use conogram_derives::Request;
use serde::Serialize;

/// Removes the profile photo of the bot. Requires no parameters. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#removemyprofilephoto)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct RemoveMyProfilePhotoParams {}

// Divider: all content below this line will be preserved after code regen
