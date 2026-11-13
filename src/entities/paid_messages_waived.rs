use serde::{Deserialize, Serialize};

/// Describes a service message about the sender of the message using a link of the other user, which allows sending messages to the user without paying Telegram Stars. Currently holds no information.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#paidmessageswaived)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaidMessagesWaived {}

// Divider: all content below this line will be preserved after code regen
