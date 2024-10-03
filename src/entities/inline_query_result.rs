use serde::Serialize;

use crate::entities::{
    inline_query_result_article::InlineQueryResultArticle,
    inline_query_result_audio::InlineQueryResultAudio,
    inline_query_result_cached_audio::InlineQueryResultCachedAudio,
    inline_query_result_cached_document::InlineQueryResultCachedDocument,
    inline_query_result_cached_gif::InlineQueryResultCachedGif,
    inline_query_result_cached_mpeg4_gif::InlineQueryResultCachedMpeg4Gif,
    inline_query_result_cached_photo::InlineQueryResultCachedPhoto,
    inline_query_result_cached_sticker::InlineQueryResultCachedSticker,
    inline_query_result_cached_video::InlineQueryResultCachedVideo,
    inline_query_result_cached_voice::InlineQueryResultCachedVoice,
    inline_query_result_contact::InlineQueryResultContact,
    inline_query_result_document::InlineQueryResultDocument,
    inline_query_result_game::InlineQueryResultGame, inline_query_result_gif::InlineQueryResultGif,
    inline_query_result_location::InlineQueryResultLocation,
    inline_query_result_mpeg4_gif::InlineQueryResultMpeg4Gif,
    inline_query_result_photo::InlineQueryResultPhoto,
    inline_query_result_venue::InlineQueryResultVenue,
    inline_query_result_video::InlineQueryResultVideo,
    inline_query_result_voice::InlineQueryResultVoice,
};

/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:
///
/// * [InlineQueryResultCachedAudio](https://core.telegram.org/bots/api/#inlinequeryresultcachedaudio)
/// * [InlineQueryResultCachedDocument](https://core.telegram.org/bots/api/#inlinequeryresultcacheddocument)
/// * [InlineQueryResultCachedGif](https://core.telegram.org/bots/api/#inlinequeryresultcachedgif)
/// * [InlineQueryResultCachedMpeg4Gif](https://core.telegram.org/bots/api/#inlinequeryresultcachedmpeg4gif)
/// * [InlineQueryResultCachedPhoto](https://core.telegram.org/bots/api/#inlinequeryresultcachedphoto)
/// * [InlineQueryResultCachedSticker](https://core.telegram.org/bots/api/#inlinequeryresultcachedsticker)
/// * [InlineQueryResultCachedVideo](https://core.telegram.org/bots/api/#inlinequeryresultcachedvideo)
/// * [InlineQueryResultCachedVoice](https://core.telegram.org/bots/api/#inlinequeryresultcachedvoice)
/// * [InlineQueryResultArticle](https://core.telegram.org/bots/api/#inlinequeryresultarticle)
/// * [InlineQueryResultAudio](https://core.telegram.org/bots/api/#inlinequeryresultaudio)
/// * [InlineQueryResultContact](https://core.telegram.org/bots/api/#inlinequeryresultcontact)
/// * [InlineQueryResultGame](https://core.telegram.org/bots/api/#inlinequeryresultgame)
/// * [InlineQueryResultDocument](https://core.telegram.org/bots/api/#inlinequeryresultdocument)
/// * [InlineQueryResultGif](https://core.telegram.org/bots/api/#inlinequeryresultgif)
/// * [InlineQueryResultLocation](https://core.telegram.org/bots/api/#inlinequeryresultlocation)
/// * [InlineQueryResultMpeg4Gif](https://core.telegram.org/bots/api/#inlinequeryresultmpeg4gif)
/// * [InlineQueryResultPhoto](https://core.telegram.org/bots/api/#inlinequeryresultphoto)
/// * [InlineQueryResultVenue](https://core.telegram.org/bots/api/#inlinequeryresultvenue)
/// * [InlineQueryResultVideo](https://core.telegram.org/bots/api/#inlinequeryresultvideo)
/// * [InlineQueryResultVoice](https://core.telegram.org/bots/api/#inlinequeryresultvoice)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresult)
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum InlineQueryResult {
    #[serde(rename = "audio")]
    CachedAudio(InlineQueryResultCachedAudio),

    #[serde(rename = "document")]
    CachedDocument(InlineQueryResultCachedDocument),

    #[serde(rename = "gif")]
    CachedGif(InlineQueryResultCachedGif),

    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),

    #[serde(rename = "photo")]
    CachedPhoto(InlineQueryResultCachedPhoto),

    #[serde(rename = "sticker")]
    CachedSticker(InlineQueryResultCachedSticker),

    #[serde(rename = "video")]
    CachedVideo(InlineQueryResultCachedVideo),

    #[serde(rename = "voice")]
    CachedVoice(InlineQueryResultCachedVoice),

    #[serde(rename = "article")]
    Article(InlineQueryResultArticle),

    #[serde(rename = "audio")]
    Audio(InlineQueryResultAudio),

    #[serde(rename = "contact")]
    Contact(InlineQueryResultContact),

    #[serde(rename = "game")]
    Game(InlineQueryResultGame),

    #[serde(rename = "document")]
    Document(InlineQueryResultDocument),

    #[serde(rename = "gif")]
    Gif(InlineQueryResultGif),

    #[serde(rename = "location")]
    Location(InlineQueryResultLocation),

    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif(InlineQueryResultMpeg4Gif),

    #[serde(rename = "photo")]
    Photo(InlineQueryResultPhoto),

    #[serde(rename = "venue")]
    Venue(InlineQueryResultVenue),

    #[serde(rename = "video")]
    Video(InlineQueryResultVideo),

    #[serde(rename = "voice")]
    Voice(InlineQueryResultVoice),
}

impl Default for InlineQueryResult {
    fn default() -> Self {
        Self::CachedAudio(InlineQueryResultCachedAudio::default())
    }
}

impl From<InlineQueryResultCachedAudio> for InlineQueryResult {
    fn from(value: InlineQueryResultCachedAudio) -> Self {
        Self::CachedAudio(value)
    }
}

impl From<InlineQueryResultCachedDocument> for InlineQueryResult {
    fn from(value: InlineQueryResultCachedDocument) -> Self {
        Self::CachedDocument(value)
    }
}

impl From<InlineQueryResultCachedGif> for InlineQueryResult {
    fn from(value: InlineQueryResultCachedGif) -> Self {
        Self::CachedGif(value)
    }
}

impl From<InlineQueryResultCachedMpeg4Gif> for InlineQueryResult {
    fn from(value: InlineQueryResultCachedMpeg4Gif) -> Self {
        Self::CachedMpeg4Gif(value)
    }
}

impl From<InlineQueryResultCachedPhoto> for InlineQueryResult {
    fn from(value: InlineQueryResultCachedPhoto) -> Self {
        Self::CachedPhoto(value)
    }
}

impl From<InlineQueryResultCachedSticker> for InlineQueryResult {
    fn from(value: InlineQueryResultCachedSticker) -> Self {
        Self::CachedSticker(value)
    }
}

impl From<InlineQueryResultCachedVideo> for InlineQueryResult {
    fn from(value: InlineQueryResultCachedVideo) -> Self {
        Self::CachedVideo(value)
    }
}

impl From<InlineQueryResultCachedVoice> for InlineQueryResult {
    fn from(value: InlineQueryResultCachedVoice) -> Self {
        Self::CachedVoice(value)
    }
}

impl From<InlineQueryResultArticle> for InlineQueryResult {
    fn from(value: InlineQueryResultArticle) -> Self {
        Self::Article(value)
    }
}

impl From<InlineQueryResultAudio> for InlineQueryResult {
    fn from(value: InlineQueryResultAudio) -> Self {
        Self::Audio(value)
    }
}

impl From<InlineQueryResultContact> for InlineQueryResult {
    fn from(value: InlineQueryResultContact) -> Self {
        Self::Contact(value)
    }
}

impl From<InlineQueryResultGame> for InlineQueryResult {
    fn from(value: InlineQueryResultGame) -> Self {
        Self::Game(value)
    }
}

impl From<InlineQueryResultDocument> for InlineQueryResult {
    fn from(value: InlineQueryResultDocument) -> Self {
        Self::Document(value)
    }
}

impl From<InlineQueryResultGif> for InlineQueryResult {
    fn from(value: InlineQueryResultGif) -> Self {
        Self::Gif(value)
    }
}

impl From<InlineQueryResultLocation> for InlineQueryResult {
    fn from(value: InlineQueryResultLocation) -> Self {
        Self::Location(value)
    }
}

impl From<InlineQueryResultMpeg4Gif> for InlineQueryResult {
    fn from(value: InlineQueryResultMpeg4Gif) -> Self {
        Self::Mpeg4Gif(value)
    }
}

impl From<InlineQueryResultPhoto> for InlineQueryResult {
    fn from(value: InlineQueryResultPhoto) -> Self {
        Self::Photo(value)
    }
}

impl From<InlineQueryResultVenue> for InlineQueryResult {
    fn from(value: InlineQueryResultVenue) -> Self {
        Self::Venue(value)
    }
}

impl From<InlineQueryResultVideo> for InlineQueryResult {
    fn from(value: InlineQueryResultVideo) -> Self {
        Self::Video(value)
    }
}

impl From<InlineQueryResultVoice> for InlineQueryResult {
    fn from(value: InlineQueryResultVoice) -> Self {
        Self::Voice(value)
    }
}

// Divider: all content below this line will be preserved after code regen
