use serde::{Deserialize, Serialize};

/// This object represents a disabled button which does nothing. Currently holds no information.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#disabledbutton)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisabledButton {}

// Divider: all content below this line will be preserved after code regen
