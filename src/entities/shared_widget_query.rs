use serde::{Deserialize, Serialize};

use crate::entities::{user::User, widget_query_type::WidgetQueryType};

/// This object represents an incoming query from a forwarded message with a shared widget created by the bot. Use [answerWidgetQuery](https://core.telegram.org/bots/api/#answerwidgetquery) to answer the query. Each user can send at most 5 queries per 5 seconds to the bot from all its widgets.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#sharedwidgetquery)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SharedWidgetQuery {
    /// Unique identifier for this query
    pub id: String,

    /// Sender
    pub from: User,

    /// Type of the query
    #[serde(rename = "type")]
    pub type_: WidgetQueryType,

    /// Data of the query, 1-64 bytes
    pub data: String,

    /// Unique identifier of the shared widget, which sent the query
    pub widget_id: String,
}

// Divider: all content below this line will be preserved after code regen
