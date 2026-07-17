use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// Caption of a rich formatted block.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockcaption)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichBlockCaption {
    /// Block caption
    pub text: Box<RichText>,

    /// *Optional*. Block credit which corresponds to the HTML tag \<cite\>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit: Option<Box<RichText>>,
}

// Divider: all content below this line will be preserved after code regen
impl RichBlockCaption {
    pub fn new(text: impl Into<RichText>, credit: Option<impl Into<RichText>>) -> Self {
        Self {
            text: Box::new(text.into()),
            credit: credit.map(Into::into).map(Box::new),
        }
    }
}
