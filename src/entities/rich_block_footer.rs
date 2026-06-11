use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A footer, corresponding to the HTML tag `<footer>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockfooter)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichBlockFooter {
    /// Text of the block
    pub text: Box<RichText>,
}

// Divider: all content below this line will be preserved after code regen
