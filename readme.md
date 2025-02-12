
<div align="center">
    <h1><b>Conogram: async Telegram Bot API client written in Rust</b></h1>
    <a href="https://core.telegram.org/bots/api#february-12-2025">
        <img src="https://img.shields.io/badge/Bot%20API%20version-8.3-brightgreen?style=for-the-badge">
    </a>
    <a href="https://crates.io/crates/conogram">
        <img src="https://img.shields.io/crates/v/conogram.svg?style=for-the-badge">
    </a>
    <br>
    <a href="https://github.com/ENCRYPTEDFOREVER/conogram/actions">
        <img src="https://github.com/ENCRYPTEDFOREVER/conogram/workflows/Build/badge.svg">
    </a>
</div>

# Aims
 - Full support of latest Bot API version (except no webhooks yet)
 - 1 to 1 API methods and entitities mapping
 - Ease and convenience of use

# Features
- Fully async
- Can be used in multithreaded context
- Full control over update handling
- Utility extension methods for _(not all yet)_ API entities _(e.g. ``Message::reply()`` method)_
- Optional automatic rate limit handling and errors caused by bot API server unavailability (``request.wrap*()``)
- Optional ChatMember cache (``Api::set_chat_member_cache_enabled(bool)``)
- Optional API calls statistics (calls count by method) ``Api::get_request_stats``
- Ability to make or not make requests based on the fact if flood wait is reached (``request.wrap_*()``)

# TODO
- More handy entity extension methods
- Webhooks support
- More examples

# Logging
[How to enable logging in your executable](https://github.com/rust-lang/log#in-executables)

# Quick usage overview

## Using [Local Bot API Server](https://core.telegram.org/bots/api#using-a-local-bot-api-server)

```rust, no_run
    let server_url = "http://localhost:80".into();
    let use_test_env = false;

    let server_config = ApiServerConfig::local(Some(server_url), use_test_env);
    let api_config = ApiConfig::new("BOT_TOKEN", Some(server_config));
    let mut api = Api::new(api_config);
    
    api.set_allowed_updates([AllowedUpdates::Message, AllowedUpdates::EditedMessage]);
    api.set_chat_member_cache_enabled(true);
    api.set_request_stats_enabled(true);
    api.set_polling_timeout(600);
```

<!-- ## Setting default [`parse_mode`](https://core.telegram.org/bots/api#formatting-options)
```rust, no_run
    let mut api = API::new(/**/);

    // Setting parse_mode for sendMessage request
    api.set_default_request_param(SendMessageRequest::get_name(), "parse_mode", "html")?;

    // For all applicable requests
    api.set_parse_mode("html")?;
``` -->

## Calling API methods
```rust, no_run
    let mut api = Api::new(todo!());
    let update: Update = todo!();

    if let Some(message) = update.message {
        // Required request parameters are in the request constructor, optionals are set via builder-like methods
        // ChatId can be username of a channel
        let request = message.reply(&api, "Text");
        // All requests implement IntoFuture
        let reply = request.await.unwrap();

        // You can handle some common API errors automatically:
        let reply = request.wrap().await.unwrap();

        // Skip sending if you've encountered flood wait already for this request
        let reply = request.wrap_nr().await.unwrap();

        // Skip sending if you have more than 2s of wait_for currently for this request
        let reply = request.wrap_nr_thr(Duration::from_secs(2)).await.unwrap();
    }
```

## Mutating and calling requests by references
```rust, no_run    
    let mut request = api.send_message(-10012345678, "Text");
    for i in 0..5 {
        request = request.chat_id(i);

        // &Request implements IntoFuture too
        let message: Message = request.wrap().await.unwrap();
    }
```

## Uploading files
```rust, no_run
let video: Message = api
    .send_video(
        "@channel_username",
        InputFile::from_file_id(
            "CgACAgQAAxkDAAI4bmekuxYmnref1iS3-BRHePlqzGg7AAKkBAACcCkVUfp03L7vloyeNgQ",
        ),
    )
    .caption("Some video")
    .wrap()
    .await
    .unwrap();

let photo: Message = api
    .send_photo("@channel_username", InputFile::from_path("my_photo.png"))
    .caption("Some photo")
    .wrap()
    .await
    .unwrap();

let media_group_messages: Vec<Message> = api
    .send_media_group(
        -1001234567,
        [
            InputFile::from_data("in_memory_file.txt", "File contents").to_document(),
            InputFile::from_path("files/some_file.txt").to_document(),
            InputFile::from_path_with_name("files/some_file.txt", "filename.txt").to_document(),
        ],
    )
    .wrap()
    .await
    .unwrap();
```

## Very-Mini-FAQ
**Q: Is it production-ready?**<br>
A: The library is used by me for a couple of years, decently polished, but is not 100% tested, some stuff may be broken, unconventional or unusable for you. 
The reason is I'm developing it for my personal use. But if you're using it too, suggestions on improvement are welcome<br><br>
