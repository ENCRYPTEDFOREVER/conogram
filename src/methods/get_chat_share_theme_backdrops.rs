use conogram_derives::Request;
use serde::Serialize;

use crate::entities::chat_share_theme_backdrops::ChatShareThemeBackdrops;

/// Use this method to get the list of backdrops, which can be used as the background of a theme of the screen for sharing a link to a user or a chat. Requires no parameters. Returns a [ChatShareThemeBackdrops](https://core.telegram.org/bots/api/#chatsharethemebackdrops) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getchatsharethemebackdrops)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = ChatShareThemeBackdrops)]
pub struct GetChatShareThemeBackdropsParams {}

// Divider: all content below this line will be preserved after code regen
