use conogram_derives::Request;
use serde::Serialize;

/// Use this method to edit public data of a shared widget created by the bot. A shared widget can be edited at most once per second. On success, *True* is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#editsharedwidgetdata)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct EditSharedWidgetDataParams {
    /// Unique identifier of the shared widget
    pub widget_id: String,

    /// New public data of the widget, 0-1024 bytes; pass an empty string to remove the public data
    pub public_data: String,

    /// The maximum amount of time in seconds that the public data of the widget may be cached client-side; 5-3600. If omitted, then the current value is kept, which is 30 by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
