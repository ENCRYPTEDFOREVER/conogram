use serde::{Deserialize, Serialize};

/// Describes a [Web App](https://core.telegram.org/bots/webapps).
///
/// API Reference: [link](https://core.telegram.org/bots/api/#webappinfo)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified in [Initializing Web Apps](https://core.telegram.org/bots/webapps#initializing-mini-apps)
    pub url: String,
}

// Divider: all content below this line will be preserved after code regen
