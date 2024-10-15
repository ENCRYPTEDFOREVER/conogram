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
    /// Represents a link to an MP3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the audio.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultcachedaudio)
    #[serde(rename = "audio")]
    CachedAudio(InlineQueryResultCachedAudio),

    /// Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the file.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultcacheddocument)
    #[serde(rename = "document")]
    CachedDocument(InlineQueryResultCachedDocument),

    /// Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with specified content instead of the animation.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultcachedgif)
    #[serde(rename = "gif")]
    CachedGif(InlineQueryResultCachedGif),

    /// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the animation.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultcachedmpeg4gif)
    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),

    /// Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the photo.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultcachedphoto)
    #[serde(rename = "photo")]
    CachedPhoto(InlineQueryResultCachedPhoto),

    /// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the sticker.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultcachedsticker)
    #[serde(rename = "sticker")]
    CachedSticker(InlineQueryResultCachedSticker),

    /// Represents a link to a video file stored on the Telegram servers. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the video.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultcachedvideo)
    #[serde(rename = "video")]
    CachedVideo(InlineQueryResultCachedVideo),

    /// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the voice message.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultcachedvoice)
    #[serde(rename = "voice")]
    CachedVoice(InlineQueryResultCachedVoice),

    /// Represents a link to an article or web page.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultarticle)
    #[serde(rename = "article")]
    Article(InlineQueryResultArticle),

    /// Represents a link to an MP3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the audio.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultaudio)
    #[serde(rename = "audio")]
    Audio(InlineQueryResultAudio),

    /// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the contact.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultcontact)
    #[serde(rename = "contact")]
    Contact(InlineQueryResultContact),

    /// Represents a [Game](https://core.telegram.org/bots/api/#games).
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultgame)
    #[serde(rename = "game")]
    Game(InlineQueryResultGame),

    /// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the file. Currently, only **.PDF** and **.ZIP** files can be sent using this method.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultdocument)
    #[serde(rename = "document")]
    Document(InlineQueryResultDocument),

    /// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the animation.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultgif)
    #[serde(rename = "gif")]
    Gif(InlineQueryResultGif),

    /// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the location.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultlocation)
    #[serde(rename = "location")]
    Location(InlineQueryResultLocation),

    /// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the animation.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultmpeg4gif)
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif(InlineQueryResultMpeg4Gif),

    /// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the photo.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultphoto)
    #[serde(rename = "photo")]
    Photo(InlineQueryResultPhoto),

    /// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the venue.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultvenue)
    #[serde(rename = "venue")]
    Venue(InlineQueryResultVenue),

    /// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the video.
    ///
    /// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you **must** replace its content using *input\_message\_content*.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultvideo)
    #[serde(rename = "video")]
    Video(InlineQueryResultVideo),

    /// Represents a link to a voice recording in an .OGG container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the the voice message.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultvoice)
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
