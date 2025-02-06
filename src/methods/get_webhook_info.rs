use conogram_derives::Request;
use serde::Serialize;

use crate::entities::webhook_info::WebhookInfo;

/// Use this method to get current webhook status. Requires no parameters. On success, returns a [WebhookInfo](https://core.telegram.org/bots/api/#webhookinfo) object. If the bot is using [getUpdates](https://core.telegram.org/bots/api/#getupdates), will return an object with the *url* field empty.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getwebhookinfo)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = WebhookInfo)]
pub struct GetWebhookInfoParams {}

// Divider: all content below this line will be preserved after code regen
