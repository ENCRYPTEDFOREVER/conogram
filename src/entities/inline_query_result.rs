use crate::entities::inline_query_result_article::InlineQueryResultArticle;
use crate::entities::inline_query_result_audio::InlineQueryResultAudio;
use crate::entities::inline_query_result_cached_audio::InlineQueryResultCachedAudio;
use crate::entities::inline_query_result_cached_document::InlineQueryResultCachedDocument;
use crate::entities::inline_query_result_cached_gif::InlineQueryResultCachedGif;
use crate::entities::inline_query_result_cached_mpeg4_gif::InlineQueryResultCachedMpeg4Gif;
use crate::entities::inline_query_result_cached_photo::InlineQueryResultCachedPhoto;
use crate::entities::inline_query_result_cached_sticker::InlineQueryResultCachedSticker;
use crate::entities::inline_query_result_cached_video::InlineQueryResultCachedVideo;
use crate::entities::inline_query_result_cached_voice::InlineQueryResultCachedVoice;
use crate::entities::inline_query_result_contact::InlineQueryResultContact;
use crate::entities::inline_query_result_document::InlineQueryResultDocument;
use crate::entities::inline_query_result_game::InlineQueryResultGame;
use crate::entities::inline_query_result_gif::InlineQueryResultGif;
use crate::entities::inline_query_result_location::InlineQueryResultLocation;
use crate::entities::inline_query_result_mpeg4_gif::InlineQueryResultMpeg4Gif;
use crate::entities::inline_query_result_photo::InlineQueryResultPhoto;
use crate::entities::inline_query_result_venue::InlineQueryResultVenue;
use crate::entities::inline_query_result_video::InlineQueryResultVideo;
use crate::entities::inline_query_result_voice::InlineQueryResultVoice;
use serde::Serialize;

///This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:
///
///* [InlineQueryResultCachedAudio](https://core.telegram.org/bots/api/#inlinequeryresultcachedaudio)
///* [InlineQueryResultCachedDocument](https://core.telegram.org/bots/api/#inlinequeryresultcacheddocument)
///* [InlineQueryResultCachedGif](https://core.telegram.org/bots/api/#inlinequeryresultcachedgif)
///* [InlineQueryResultCachedMpeg4Gif](https://core.telegram.org/bots/api/#inlinequeryresultcachedmpeg4gif)
///* [InlineQueryResultCachedPhoto](https://core.telegram.org/bots/api/#inlinequeryresultcachedphoto)
///* [InlineQueryResultCachedSticker](https://core.telegram.org/bots/api/#inlinequeryresultcachedsticker)
///* [InlineQueryResultCachedVideo](https://core.telegram.org/bots/api/#inlinequeryresultcachedvideo)
///* [InlineQueryResultCachedVoice](https://core.telegram.org/bots/api/#inlinequeryresultcachedvoice)
///* [InlineQueryResultArticle](https://core.telegram.org/bots/api/#inlinequeryresultarticle)
///* [InlineQueryResultAudio](https://core.telegram.org/bots/api/#inlinequeryresultaudio)
///* [InlineQueryResultContact](https://core.telegram.org/bots/api/#inlinequeryresultcontact)
///* [InlineQueryResultGame](https://core.telegram.org/bots/api/#inlinequeryresultgame)
///* [InlineQueryResultDocument](https://core.telegram.org/bots/api/#inlinequeryresultdocument)
///* [InlineQueryResultGif](https://core.telegram.org/bots/api/#inlinequeryresultgif)
///* [InlineQueryResultLocation](https://core.telegram.org/bots/api/#inlinequeryresultlocation)
///* [InlineQueryResultMpeg4Gif](https://core.telegram.org/bots/api/#inlinequeryresultmpeg4gif)
///* [InlineQueryResultPhoto](https://core.telegram.org/bots/api/#inlinequeryresultphoto)
///* [InlineQueryResultVenue](https://core.telegram.org/bots/api/#inlinequeryresultvenue)
///* [InlineQueryResultVideo](https://core.telegram.org/bots/api/#inlinequeryresultvideo)
///* [InlineQueryResultVoice](https://core.telegram.org/bots/api/#inlinequeryresultvoice)
///API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresult)
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
// Divider: all content below this line will be preserved after code regen
