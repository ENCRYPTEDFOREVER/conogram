use conogram_derives::Request;
use serde::Serialize;

use crate::entities::sticker::Sticker;

/// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getforumtopiciconstickers)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Vec<Sticker>)]
pub struct GetForumTopicIconStickersParams {}

// Divider: all content below this line will be preserved after code regen
