use serde::Serialize;

use crate::entities::{
    input_rich_block_anchor::InputRichBlockAnchor,
    input_rich_block_animation::InputRichBlockAnimation,
    input_rich_block_audio::InputRichBlockAudio,
    input_rich_block_block_quotation::InputRichBlockBlockQuotation,
    input_rich_block_collage::InputRichBlockCollage,
    input_rich_block_details::InputRichBlockDetails,
    input_rich_block_divider::InputRichBlockDivider, input_rich_block_footer::InputRichBlockFooter,
    input_rich_block_list::InputRichBlockList, input_rich_block_map::InputRichBlockMap,
    input_rich_block_mathematical_expression::InputRichBlockMathematicalExpression,
    input_rich_block_paragraph::InputRichBlockParagraph,
    input_rich_block_photo::InputRichBlockPhoto,
    input_rich_block_preformatted::InputRichBlockPreformatted,
    input_rich_block_pull_quotation::InputRichBlockPullQuotation,
    input_rich_block_section_heading::InputRichBlockSectionHeading,
    input_rich_block_slideshow::InputRichBlockSlideshow,
    input_rich_block_table::InputRichBlockTable, input_rich_block_thinking::InputRichBlockThinking,
    input_rich_block_video::InputRichBlockVideo,
    input_rich_block_voice_note::InputRichBlockVoiceNote,
};

/// This object represents a block in a rich formatted message to be sent. Currently, it can be any of the following types:
///
/// * [InputRichBlockParagraph](https://core.telegram.org/bots/api/#inputrichblockparagraph)
/// * [InputRichBlockSectionHeading](https://core.telegram.org/bots/api/#inputrichblocksectionheading)
/// * [InputRichBlockPreformatted](https://core.telegram.org/bots/api/#inputrichblockpreformatted)
/// * [InputRichBlockFooter](https://core.telegram.org/bots/api/#inputrichblockfooter)
/// * [InputRichBlockDivider](https://core.telegram.org/bots/api/#inputrichblockdivider)
/// * [InputRichBlockMathematicalExpression](https://core.telegram.org/bots/api/#inputrichblockmathematicalexpression)
/// * [InputRichBlockAnchor](https://core.telegram.org/bots/api/#inputrichblockanchor)
/// * [InputRichBlockList](https://core.telegram.org/bots/api/#inputrichblocklist)
/// * [InputRichBlockBlockQuotation](https://core.telegram.org/bots/api/#inputrichblockblockquotation)
/// * [InputRichBlockPullQuotation](https://core.telegram.org/bots/api/#inputrichblockpullquotation)
/// * [InputRichBlockCollage](https://core.telegram.org/bots/api/#inputrichblockcollage)
/// * [InputRichBlockSlideshow](https://core.telegram.org/bots/api/#inputrichblockslideshow)
/// * [InputRichBlockTable](https://core.telegram.org/bots/api/#inputrichblocktable)
/// * [InputRichBlockDetails](https://core.telegram.org/bots/api/#inputrichblockdetails)
/// * [InputRichBlockMap](https://core.telegram.org/bots/api/#inputrichblockmap)
/// * [InputRichBlockAnimation](https://core.telegram.org/bots/api/#inputrichblockanimation)
/// * [InputRichBlockAudio](https://core.telegram.org/bots/api/#inputrichblockaudio)
/// * [InputRichBlockPhoto](https://core.telegram.org/bots/api/#inputrichblockphoto)
/// * [InputRichBlockVideo](https://core.telegram.org/bots/api/#inputrichblockvideo)
/// * [InputRichBlockVoiceNote](https://core.telegram.org/bots/api/#inputrichblockvoicenote)
/// * [InputRichBlockThinking](https://core.telegram.org/bots/api/#inputrichblockthinking)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblock)
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum InputRichBlock {
    /// A text paragraph, corresponding to the HTML tag `<p>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockparagraph)
    #[serde(rename = "paragraph")]
    Paragraph(InputRichBlockParagraph),

    /// A section heading, corresponding to the HTML tags `<h1>`, `<h2>`, `<h3>`, `<h4>`, `<h5>`, or `<h6>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblocksectionheading)
    #[serde(rename = "heading")]
    SectionHeading(InputRichBlockSectionHeading),

    /// A preformatted text block, corresponding to the nested HTML tags `<pre>` and `<code>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockpreformatted)
    #[serde(rename = "pre")]
    Preformatted(InputRichBlockPreformatted),

    /// A footer, corresponding to the HTML tag `<footer>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockfooter)
    #[serde(rename = "footer")]
    Footer(InputRichBlockFooter),

    /// A divider, corresponding to the HTML tag `<hr/>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockdivider)
    #[serde(rename = "divider")]
    Divider(InputRichBlockDivider),

    /// A block with a mathematical expression in LaTeX format, corresponding to the custom HTML tag `<tg-math-block>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockmathematicalexpression)
    #[serde(rename = "mathematical_expression")]
    MathematicalExpression(InputRichBlockMathematicalExpression),

    /// A block with an anchor, corresponding to the HTML tag `<a>` with the attribute `name`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockanchor)
    #[serde(rename = "anchor")]
    Anchor(InputRichBlockAnchor),

    /// A list of blocks, corresponding to the HTML tag `<ul>` or `<ol>` with multiple nested tags `<li>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblocklist)
    #[serde(rename = "list")]
    List(InputRichBlockList),

    /// A block quotation, corresponding to the HTML tag `<blockquote>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockblockquotation)
    #[serde(rename = "blockquote")]
    BlockQuotation(InputRichBlockBlockQuotation),

    /// A quotation with centered text, loosely corresponding to the HTML tag `<aside>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockpullquotation)
    #[serde(rename = "pullquote")]
    PullQuotation(InputRichBlockPullQuotation),

    /// A collage, corresponding to the custom HTML tag `<tg-collage>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockcollage)
    #[serde(rename = "collage")]
    Collage(InputRichBlockCollage),

    /// A slideshow, corresponding to the custom HTML tag `<tg-slideshow>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockslideshow)
    #[serde(rename = "slideshow")]
    Slideshow(InputRichBlockSlideshow),

    /// A table, corresponding to the HTML tag `<table>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblocktable)
    #[serde(rename = "table")]
    Table(InputRichBlockTable),

    /// An expandable block for details disclosure, corresponding to the HTML tag `<details>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockdetails)
    #[serde(rename = "details")]
    Details(InputRichBlockDetails),

    /// A block with a map, corresponding to the custom HTML tag `<tg-map>`. The map's width and height must not exceed 10000 in total. The width and height ratio must be at most 20.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockmap)
    #[serde(rename = "map")]
    Map(InputRichBlockMap),

    /// A block with an animation, corresponding to the HTML tag `<video>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockanimation)
    #[serde(rename = "animation")]
    Animation(InputRichBlockAnimation),

    /// A block with a music file, corresponding to the HTML tag `<audio>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockaudio)
    #[serde(rename = "audio")]
    Audio(InputRichBlockAudio),

    /// A block with a photo, corresponding to the HTML tag `<img>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockphoto)
    #[serde(rename = "photo")]
    Photo(InputRichBlockPhoto),

    /// A block with a video, corresponding to the HTML tag `<video>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockvideo)
    #[serde(rename = "video")]
    Video(InputRichBlockVideo),

    /// A block with a voice note, corresponding to the HTML tag `<audio>`.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockvoicenote)
    #[serde(rename = "voice_note")]
    VoiceNote(InputRichBlockVoiceNote),

    /// A block with a “Thinking…” placeholder, corresponding to the custom HTML tag `<tg-thinking>`. The block may be used only in [sendRichMessageDraft](https://core.telegram.org/bots/api/#sendrichmessagedraft), therefore it can't be received in messages. See [https://t.me/addemoji/AIActions](https://t.me/addemoji/AIActions) for examples of custom emoji that are recommended for usage in the block.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockthinking)
    #[serde(rename = "thinking")]
    Thinking(InputRichBlockThinking),
}

impl Default for InputRichBlock {
    fn default() -> Self {
        Self::Paragraph(InputRichBlockParagraph::default())
    }
}

impl From<InputRichBlockParagraph> for InputRichBlock {
    fn from(value: InputRichBlockParagraph) -> Self {
        Self::Paragraph(value)
    }
}

impl From<InputRichBlockSectionHeading> for InputRichBlock {
    fn from(value: InputRichBlockSectionHeading) -> Self {
        Self::SectionHeading(value)
    }
}

impl From<InputRichBlockPreformatted> for InputRichBlock {
    fn from(value: InputRichBlockPreformatted) -> Self {
        Self::Preformatted(value)
    }
}

impl From<InputRichBlockFooter> for InputRichBlock {
    fn from(value: InputRichBlockFooter) -> Self {
        Self::Footer(value)
    }
}

impl From<InputRichBlockDivider> for InputRichBlock {
    fn from(value: InputRichBlockDivider) -> Self {
        Self::Divider(value)
    }
}

impl From<InputRichBlockMathematicalExpression> for InputRichBlock {
    fn from(value: InputRichBlockMathematicalExpression) -> Self {
        Self::MathematicalExpression(value)
    }
}

impl From<InputRichBlockAnchor> for InputRichBlock {
    fn from(value: InputRichBlockAnchor) -> Self {
        Self::Anchor(value)
    }
}

impl From<InputRichBlockList> for InputRichBlock {
    fn from(value: InputRichBlockList) -> Self {
        Self::List(value)
    }
}

impl From<InputRichBlockBlockQuotation> for InputRichBlock {
    fn from(value: InputRichBlockBlockQuotation) -> Self {
        Self::BlockQuotation(value)
    }
}

impl From<InputRichBlockPullQuotation> for InputRichBlock {
    fn from(value: InputRichBlockPullQuotation) -> Self {
        Self::PullQuotation(value)
    }
}

impl From<InputRichBlockCollage> for InputRichBlock {
    fn from(value: InputRichBlockCollage) -> Self {
        Self::Collage(value)
    }
}

impl From<InputRichBlockSlideshow> for InputRichBlock {
    fn from(value: InputRichBlockSlideshow) -> Self {
        Self::Slideshow(value)
    }
}

impl From<InputRichBlockTable> for InputRichBlock {
    fn from(value: InputRichBlockTable) -> Self {
        Self::Table(value)
    }
}

impl From<InputRichBlockDetails> for InputRichBlock {
    fn from(value: InputRichBlockDetails) -> Self {
        Self::Details(value)
    }
}

impl From<InputRichBlockMap> for InputRichBlock {
    fn from(value: InputRichBlockMap) -> Self {
        Self::Map(value)
    }
}

impl From<InputRichBlockAnimation> for InputRichBlock {
    fn from(value: InputRichBlockAnimation) -> Self {
        Self::Animation(value)
    }
}

impl From<InputRichBlockAudio> for InputRichBlock {
    fn from(value: InputRichBlockAudio) -> Self {
        Self::Audio(value)
    }
}

impl From<InputRichBlockPhoto> for InputRichBlock {
    fn from(value: InputRichBlockPhoto) -> Self {
        Self::Photo(value)
    }
}

impl From<InputRichBlockVideo> for InputRichBlock {
    fn from(value: InputRichBlockVideo) -> Self {
        Self::Video(value)
    }
}

impl From<InputRichBlockVoiceNote> for InputRichBlock {
    fn from(value: InputRichBlockVoiceNote) -> Self {
        Self::VoiceNote(value)
    }
}

impl From<InputRichBlockThinking> for InputRichBlock {
    fn from(value: InputRichBlockThinking) -> Self {
        Self::Thinking(value)
    }
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::{
    input_media_animation::InputMediaAnimation, input_media_audio::InputMediaAudio,
    input_media_photo::InputMediaPhoto, input_media_video::InputMediaVideo,
    input_media_voice_note::InputMediaVoiceNote,
    input_rich_block_list_item::InputRichBlockListItem, location::Location,
    misc::input_file::GetFiles, rich_block_caption::RichBlockCaption,
    rich_block_table_cell::RichBlockTableCell, rich_text::RichText,
};

impl GetFiles for InputRichBlock {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.getfiles_helper(form).await
    }
}

impl InputRichBlock {
    /// This is needed to break out of the recursion cycle
    fn getfiles_helper<'a>(
        &'a self,
        form: reqwest::multipart::Form,
    ) -> std::pin::Pin<
        Box<
            dyn Future<Output = Result<reqwest::multipart::Form, std::io::Error>>
                + Send
                + Sync
                + 'a,
        >,
    > {
        Box::pin(async move {
            match self {
                Self::List(v) => v.form(form).await,
                Self::BlockQuotation(v) => v.form(form).await,
                Self::Collage(v) => v.form(form).await,
                Self::Slideshow(v) => v.form(form).await,
                Self::Details(v) => v.form(form).await,

                Self::Animation(v) => v.form(form).await,
                Self::Audio(v) => v.form(form).await,
                Self::Photo(v) => v.form(form).await,
                Self::Video(v) => v.form(form).await,
                Self::VoiceNote(v) => v.form(form).await,

                _ => Ok(form),
            }
        })
    }

    pub fn paragraph(text: impl Into<RichText>) -> Self {
        InputRichBlockParagraph {
            text: Box::new(text.into()),
        }
        .into()
    }

    pub fn section_heading(text: impl Into<RichText>, size: impl Into<i64>) -> Self {
        InputRichBlockSectionHeading {
            text: Box::new(text.into()),
            size: size.into(),
        }
        .into()
    }

    pub fn preformatted(text: impl Into<RichText>, language: Option<impl Into<String>>) -> Self {
        InputRichBlockPreformatted {
            text: Box::new(text.into()),
            language: language.map(Into::into),
        }
        .into()
    }

    pub fn footer(text: impl Into<RichText>) -> Self {
        InputRichBlockFooter {
            text: Box::new(text.into()),
        }
        .into()
    }

    #[must_use]
    pub fn divider() -> Self {
        InputRichBlockDivider {}.into()
    }

    pub fn mathematical_expression(expression: impl Into<String>) -> Self {
        InputRichBlockMathematicalExpression {
            expression: expression.into(),
        }
        .into()
    }

    pub fn anchor(name: impl Into<String>) -> Self {
        InputRichBlockAnchor { name: name.into() }.into()
    }

    pub fn list(items: impl IntoIterator<Item = impl Into<InputRichBlockListItem>>) -> Self {
        InputRichBlockList {
            items: items.into_iter().map(Into::into).collect(),
        }
        .into()
    }

    pub fn block_quotation(
        blocks: impl IntoIterator<Item = impl Into<Self>>,
        credit: Option<impl Into<RichText>>,
    ) -> Self {
        InputRichBlockBlockQuotation {
            blocks: blocks.into_iter().map(Into::into).collect(),
            credit: credit.map(Into::into).map(Box::new),
        }
        .into()
    }

    pub fn pull_quotation(text: impl Into<RichText>, credit: Option<impl Into<RichText>>) -> Self {
        InputRichBlockPullQuotation {
            text: Box::new(text.into()),
            credit: credit.map(Into::into).map(Box::new),
        }
        .into()
    }

    pub fn collage(
        blocks: impl IntoIterator<Item = impl Into<Self>>,
        caption: Option<RichBlockCaption>,
    ) -> Self {
        InputRichBlockCollage {
            blocks: blocks.into_iter().map(Into::into).collect(),
            caption,
        }
        .into()
    }

    pub fn slideshow(
        blocks: impl IntoIterator<Item = impl Into<Self>>,
        caption: Option<RichBlockCaption>,
    ) -> Self {
        InputRichBlockSlideshow {
            blocks: blocks.into_iter().map(Into::into).collect(),
            caption,
        }
        .into()
    }

    pub fn table(
        cells: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<RichBlockTableCell>>>,
        caption: Option<impl Into<RichText>>,
        is_bordered: bool,
        is_striped: bool,
    ) -> Self {
        InputRichBlockTable {
            cells: cells
                .into_iter()
                .map(|row| row.into_iter().map(Into::into).collect())
                .collect(),
            caption: caption.map(Into::into).map(Box::new),

            is_bordered,
            is_striped,
        }
        .into()
    }

    pub fn details(
        summary: impl Into<RichText>,
        blocks: impl IntoIterator<Item = impl Into<Self>>,
        is_open: bool,
    ) -> Self {
        InputRichBlockDetails {
            summary: Box::new(summary.into()),
            blocks: blocks.into_iter().map(Into::into).collect(),

            is_open,
        }
        .into()
    }

    pub fn map(
        location: impl Into<Location>,
        zoom: impl Into<i64>,
        width: impl Into<i64>,
        height: impl Into<i64>,
        caption: Option<RichBlockCaption>,
    ) -> Self {
        InputRichBlockMap {
            location: location.into(),
            zoom: zoom.into(),
            width: width.into(),
            height: height.into(),

            caption,
        }
        .into()
    }

    pub fn animation(
        animation: impl Into<InputMediaAnimation>,
        caption: Option<RichBlockCaption>,
    ) -> Self {
        InputRichBlockAnimation {
            animation: animation.into(),
            caption,
        }
        .into()
    }

    pub fn audio(audio: impl Into<InputMediaAudio>, caption: Option<RichBlockCaption>) -> Self {
        InputRichBlockAudio {
            audio: audio.into(),
            caption,
        }
        .into()
    }

    pub fn photo(photo: impl Into<InputMediaPhoto>, caption: Option<RichBlockCaption>) -> Self {
        InputRichBlockPhoto {
            photo: photo.into(),
            caption,
        }
        .into()
    }

    pub fn video(video: impl Into<InputMediaVideo>, caption: Option<RichBlockCaption>) -> Self {
        InputRichBlockVideo {
            video: video.into(),
            caption,
        }
        .into()
    }

    pub fn voice_note(
        voice_note: impl Into<InputMediaVoiceNote>,
        caption: Option<RichBlockCaption>,
    ) -> Self {
        InputRichBlockVoiceNote {
            voice_note: voice_note.into(),
            caption,
        }
        .into()
    }

    pub fn thinking(text: impl Into<RichText>) -> Self {
        InputRichBlockThinking {
            text: Box::new(text.into()),
        }
        .into()
    }
}
