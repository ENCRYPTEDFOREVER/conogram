use serde::{Deserialize, Serialize};

use crate::entities::{
    rich_block_anchor::RichBlockAnchor, rich_block_animation::RichBlockAnimation,
    rich_block_audio::RichBlockAudio, rich_block_block_quotation::RichBlockBlockQuotation,
    rich_block_buttons::RichBlockButtons, rich_block_collage::RichBlockCollage,
    rich_block_details::RichBlockDetails, rich_block_divider::RichBlockDivider,
    rich_block_document::RichBlockDocument,
    rich_block_expandable_block_quotation::RichBlockExpandableBlockQuotation,
    rich_block_footer::RichBlockFooter, rich_block_list::RichBlockList,
    rich_block_map::RichBlockMap,
    rich_block_mathematical_expression::RichBlockMathematicalExpression,
    rich_block_paragraph::RichBlockParagraph, rich_block_photo::RichBlockPhoto,
    rich_block_preformatted::RichBlockPreformatted,
    rich_block_pull_quotation::RichBlockPullQuotation,
    rich_block_section_heading::RichBlockSectionHeading, rich_block_slideshow::RichBlockSlideshow,
    rich_block_table::RichBlockTable, rich_block_thinking::RichBlockThinking,
    rich_block_video::RichBlockVideo, rich_block_voice_note::RichBlockVoiceNote,
};

/// This object represents a block in a rich formatted message. Currently, it can be any of the following types:
///
/// * [RichBlockParagraph](https://core.telegram.org/bots/api/#richblockparagraph)
/// * [RichBlockSectionHeading](https://core.telegram.org/bots/api/#richblocksectionheading)
/// * [RichBlockPreformatted](https://core.telegram.org/bots/api/#richblockpreformatted)
/// * [RichBlockFooter](https://core.telegram.org/bots/api/#richblockfooter)
/// * [RichBlockDivider](https://core.telegram.org/bots/api/#richblockdivider)
/// * [RichBlockMathematicalExpression](https://core.telegram.org/bots/api/#richblockmathematicalexpression)
/// * [RichBlockAnchor](https://core.telegram.org/bots/api/#richblockanchor)
/// * [RichBlockList](https://core.telegram.org/bots/api/#richblocklist)
/// * [RichBlockBlockQuotation](https://core.telegram.org/bots/api/#richblockblockquotation)
/// * [RichBlockExpandableBlockQuotation](https://core.telegram.org/bots/api/#richblockexpandableblockquotation)
/// * [RichBlockPullQuotation](https://core.telegram.org/bots/api/#richblockpullquotation)
/// * [RichBlockCollage](https://core.telegram.org/bots/api/#richblockcollage)
/// * [RichBlockSlideshow](https://core.telegram.org/bots/api/#richblockslideshow)
/// * [RichBlockTable](https://core.telegram.org/bots/api/#richblocktable)
/// * [RichBlockDetails](https://core.telegram.org/bots/api/#richblockdetails)
/// * [RichBlockMap](https://core.telegram.org/bots/api/#richblockmap)
/// * [RichBlockButtons](https://core.telegram.org/bots/api/#richblockbuttons)
/// * [RichBlockAnimation](https://core.telegram.org/bots/api/#richblockanimation)
/// * [RichBlockAudio](https://core.telegram.org/bots/api/#richblockaudio)
/// * [RichBlockDocument](https://core.telegram.org/bots/api/#richblockdocument)
/// * [RichBlockPhoto](https://core.telegram.org/bots/api/#richblockphoto)
/// * [RichBlockVideo](https://core.telegram.org/bots/api/#richblockvideo)
/// * [RichBlockVoiceNote](https://core.telegram.org/bots/api/#richblockvoicenote)
/// * [RichBlockThinking](https://core.telegram.org/bots/api/#richblockthinking)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblock)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RichBlock {
    /// A text paragraph, corresponding to the HTML tag `<p>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockparagraph)
    Paragraph(RichBlockParagraph),

    /// A section heading, corresponding to the HTML tags `<h1>`, `<h2>`, `<h3>`, `<h4>`, `<h5>`, or `<h6>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblocksectionheading)
    SectionHeading(RichBlockSectionHeading),

    /// A preformatted text block, corresponding to the nested HTML tags `<pre>` and `<code>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockpreformatted)
    Preformatted(RichBlockPreformatted),

    /// A footer, corresponding to the HTML tag `<footer>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockfooter)
    Footer(RichBlockFooter),

    /// A divider, corresponding to the HTML tag `<hr/>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockdivider)
    Divider(RichBlockDivider),

    /// A block with a mathematical expression in LaTeX format, corresponding to the custom HTML tag `<tg-math-block>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockmathematicalexpression)
    MathematicalExpression(RichBlockMathematicalExpression),

    /// A block with an anchor, corresponding to the HTML tag `<a>` with the attribute `name`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockanchor)
    Anchor(RichBlockAnchor),

    /// A list of blocks, corresponding to the HTML tag `<ul>` or `<ol>` with multiple nested tags `<li>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblocklist)
    List(RichBlockList),

    /// A block quotation, corresponding to the HTML tag `<blockquote>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockblockquotation)
    BlockQuotation(RichBlockBlockQuotation),

    /// A block quotation, corresponding to the HTML tag `<blockquote>` with custom attribute `"expandable"`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockexpandableblockquotation)
    ExpandableBlockQuotation(RichBlockExpandableBlockQuotation),

    /// A quotation with centered text, loosely corresponding to the HTML tag `<aside>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockpullquotation)
    PullQuotation(RichBlockPullQuotation),

    /// A collage, corresponding to the custom HTML tag `<tg-collage>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockcollage)
    Collage(RichBlockCollage),

    /// A slideshow, corresponding to the custom HTML tag `<tg-slideshow>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockslideshow)
    Slideshow(RichBlockSlideshow),

    /// A table, corresponding to the HTML tag `<table>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblocktable)
    Table(RichBlockTable),

    /// An expandable block for details disclosure, corresponding to the HTML tag `<details>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockdetails)
    Details(RichBlockDetails),

    /// A block with a map, corresponding to the custom HTML tag `<tg-map>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockmap)
    Map(RichBlockMap),

    /// A block containing a list of buttons that are shown in one row, corresponding to the custom HTML tag `<tg-button-row>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockbuttons)
    Buttons(RichBlockButtons),

    /// A block with an animation, corresponding to the HTML tag `<video>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockanimation)
    Animation(RichBlockAnimation),

    /// A block with a music file, corresponding to the HTML tag `<audio>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockaudio)
    Audio(RichBlockAudio),

    /// A block with a general file, corresponding to the custom HTML tag `<tg-document>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockdocument)
    Document(RichBlockDocument),

    /// A block with a photo, corresponding to the HTML tag `<img>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockphoto)
    Photo(RichBlockPhoto),

    /// A block with a video, corresponding to the HTML tag `<video>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockvideo)
    Video(RichBlockVideo),

    /// A block with a voice note, corresponding to the HTML tag `<audio>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockvoicenote)
    VoiceNote(RichBlockVoiceNote),

    /// A block with a “Thinking…” placeholder, corresponding to the custom HTML tag `<tg-thinking>`. The block may be used only in [sendRichMessageDraft](https://core.telegram.org/bots/api/#sendrichmessagedraft), therefore it can't be received in messages. See [https://t.me/addemoji/AIActions](https://t.me/addemoji/AIActions) for examples of custom emoji that are recommended for usage in the block.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#richblockthinking)
    Thinking(RichBlockThinking),
}

impl Default for RichBlock {
    fn default() -> Self {
        Self::Paragraph(RichBlockParagraph::default())
    }
}

impl From<RichBlockParagraph> for RichBlock {
    fn from(value: RichBlockParagraph) -> Self {
        Self::Paragraph(value)
    }
}

impl From<RichBlockSectionHeading> for RichBlock {
    fn from(value: RichBlockSectionHeading) -> Self {
        Self::SectionHeading(value)
    }
}

impl From<RichBlockPreformatted> for RichBlock {
    fn from(value: RichBlockPreformatted) -> Self {
        Self::Preformatted(value)
    }
}

impl From<RichBlockFooter> for RichBlock {
    fn from(value: RichBlockFooter) -> Self {
        Self::Footer(value)
    }
}

impl From<RichBlockDivider> for RichBlock {
    fn from(value: RichBlockDivider) -> Self {
        Self::Divider(value)
    }
}

impl From<RichBlockMathematicalExpression> for RichBlock {
    fn from(value: RichBlockMathematicalExpression) -> Self {
        Self::MathematicalExpression(value)
    }
}

impl From<RichBlockAnchor> for RichBlock {
    fn from(value: RichBlockAnchor) -> Self {
        Self::Anchor(value)
    }
}

impl From<RichBlockList> for RichBlock {
    fn from(value: RichBlockList) -> Self {
        Self::List(value)
    }
}

impl From<RichBlockBlockQuotation> for RichBlock {
    fn from(value: RichBlockBlockQuotation) -> Self {
        Self::BlockQuotation(value)
    }
}

impl From<RichBlockExpandableBlockQuotation> for RichBlock {
    fn from(value: RichBlockExpandableBlockQuotation) -> Self {
        Self::ExpandableBlockQuotation(value)
    }
}

impl From<RichBlockPullQuotation> for RichBlock {
    fn from(value: RichBlockPullQuotation) -> Self {
        Self::PullQuotation(value)
    }
}

impl From<RichBlockCollage> for RichBlock {
    fn from(value: RichBlockCollage) -> Self {
        Self::Collage(value)
    }
}

impl From<RichBlockSlideshow> for RichBlock {
    fn from(value: RichBlockSlideshow) -> Self {
        Self::Slideshow(value)
    }
}

impl From<RichBlockTable> for RichBlock {
    fn from(value: RichBlockTable) -> Self {
        Self::Table(value)
    }
}

impl From<RichBlockDetails> for RichBlock {
    fn from(value: RichBlockDetails) -> Self {
        Self::Details(value)
    }
}

impl From<RichBlockMap> for RichBlock {
    fn from(value: RichBlockMap) -> Self {
        Self::Map(value)
    }
}

impl From<RichBlockButtons> for RichBlock {
    fn from(value: RichBlockButtons) -> Self {
        Self::Buttons(value)
    }
}

impl From<RichBlockAnimation> for RichBlock {
    fn from(value: RichBlockAnimation) -> Self {
        Self::Animation(value)
    }
}

impl From<RichBlockAudio> for RichBlock {
    fn from(value: RichBlockAudio) -> Self {
        Self::Audio(value)
    }
}

impl From<RichBlockDocument> for RichBlock {
    fn from(value: RichBlockDocument) -> Self {
        Self::Document(value)
    }
}

impl From<RichBlockPhoto> for RichBlock {
    fn from(value: RichBlockPhoto) -> Self {
        Self::Photo(value)
    }
}

impl From<RichBlockVideo> for RichBlock {
    fn from(value: RichBlockVideo) -> Self {
        Self::Video(value)
    }
}

impl From<RichBlockVoiceNote> for RichBlock {
    fn from(value: RichBlockVoiceNote) -> Self {
        Self::VoiceNote(value)
    }
}

impl From<RichBlockThinking> for RichBlock {
    fn from(value: RichBlockThinking) -> Self {
        Self::Thinking(value)
    }
}

// Divider: all content below this line will be preserved after code regen
