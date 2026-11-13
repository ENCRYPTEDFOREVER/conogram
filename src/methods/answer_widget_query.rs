use conogram_derives::Request;
use serde::Serialize;

/// Use this method to send an answer to a query from a widget. The query must be answered within 15 seconds. The answer is received only by the widget that sent the query and doesn't change data of the widget; use [editMessageWidgetData](https://core.telegram.org/bots/api/#editmessagewidgetdata) or [editSharedWidgetData](https://core.telegram.org/bots/api/#editsharedwidgetdata) to change it. On success, *True* is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#answerwidgetquery)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct AnswerWidgetQueryParams {
    /// Unique identifier for the query to be answered
    pub widget_query_id: String,

    /// Data to be returned to the widget, 0-1024 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
