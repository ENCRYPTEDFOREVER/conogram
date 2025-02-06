use conogram_derives::Request;
use serde::Serialize;

use crate::entities::business_connection::BusinessConnection;

/// Use this method to get information about the connection of the bot with a business account. Returns a [BusinessConnection](https://core.telegram.org/bots/api/#businessconnection) object on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getbusinessconnection)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = BusinessConnection)]
pub struct GetBusinessConnectionParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
}

// Divider: all content below this line will be preserved after code regen
