use conogram_derives::Request;
use serde::Serialize;

use crate::entities::accepted_gift_types::AcceptedGiftTypes;

/// Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the *can\_change\_gift\_settings* business bot right. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setbusinessaccountgiftsettings)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetBusinessAccountGiftSettingsParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// Pass True, if a button for sending a gift to the user or by the business account must always be shown in the input field
    pub show_gift_button: bool,

    /// Types of gifts accepted by the business account
    pub accepted_gift_types: AcceptedGiftTypes,
}

// Divider: all content below this line will be preserved after code regen
