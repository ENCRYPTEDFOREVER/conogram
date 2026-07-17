use serde::Serialize;

use crate::entities::rich_text::RichText;

/// A preformatted text block, corresponding to the nested HTML tags `<pre>` and `<code>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockpreformatted)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "pre", tag = "type")]
pub struct InputRichBlockPreformatted {
    /// Text of the block
    pub text: Box<RichText>,

    /// *Optional*. The programming language of the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
