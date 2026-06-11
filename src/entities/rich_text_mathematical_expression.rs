use serde::{Deserialize, Serialize};

/// A mathematical expression.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextmathematicalexpression)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RichTextMathematicalExpression {
    /// The expression in LaTeX format
    pub expression: String,
}

// Divider: all content below this line will be preserved after code regen
