use serde::{Deserialize, Serialize};

/// A block with a mathematical expression in LaTeX format, corresponding to the custom HTML tag `<tg-math-block>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockmathematicalexpression)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "mathematical_expression", tag = "type")]
pub struct RichBlockMathematicalExpression {
    /// The mathematical expression in LaTeX format
    pub expression: String,
}

// Divider: all content below this line will be preserved after code regen
