use conogram_derives::Request;
use serde::Serialize;

/// Use this method to process a received chat join request query by showing a Mini App to the user before deciding the outcome. Call [answerChatJoinRequestQuery](https://core.telegram.org/bots/api/#answerchatjoinrequestquery) to resolve the join request query based on the user interaction with the Mini App. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#sendchatjoinrequestwebapp)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SendChatJoinRequestWebAppParams {
    /// Unique identifier of the join request query
    pub chat_join_request_query_id: String,

    /// An HTTPS URL of a Web App to be opened with additional data as specified in [Initializing Web Apps](https://core.telegram.org/bots/webapps#initializing-mini-apps)
    pub web_app_url: String,
}

// Divider: all content below this line will be preserved after code regen
