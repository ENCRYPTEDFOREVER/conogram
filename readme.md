
<div align="center">
    <h1><b>Conogram: async Telegram Bot API client written in Rust</b></h1>
    <a href="https://github.com/ENCRYPTEDFOREVER/conogram/actions">
        <img src="https://github.com/ENCRYPTEDFOREVER/conogram/workflows/Build/badge.svg">
    </a>
    <a href="https://core.telegram.org/bots/api#april-21-2023">
        <img src="https://img.shields.io/badge/Bot%20API%20version-6.7-brightgreen">
    </a>
</div>

# Aims
 - Full support of latest Bot API version _[no webhooks yet](#webhooks-support)_
 - 1 to 1 API methods and entitities mapping
 - Ease of use
 

# Features
- Fully async
- Utility extension methods for API entities _(e.g. ``message.reply`` method)_
- Ability to set default request params _(e.g. ``parse_mode`` or ``allow_sending_without_reply``)_
- Can be used in multithreaded context
- Full control over update handling

# TODO
- More handy entity methods
- #### Webhooks support
- Logging
- More examples


# Quick usage overview

## Using [Local Bot API Server](https://core.telegram.org/bots/api#using-a-local-bot-api-server)

```rust, no_run
    let server_config = ApiServerConfig::local("http://localhost", 80, true);

    let api_config = APIConfig::new("BOT_TOKEN", server_config)?

    let api = API::new(api_config);
```

## Setting default [`parse_mode`](https://core.telegram.org/bots/api#formatting-options)
```rust, no_run
    let mut api = API::new(/**/);

    // Setting parse_mode for sendMessage request
    api.set_default_request_param(SendMessageRequest::get_name(), "parse_mode", "html")?;

    // For all applicable requests
    api.set_parse_mode("html")?;
```

## Calling API methods
```rust, no_run
    let mut api = API::new(/**/);

    // Required request parameters are in the request constructor, optionals are set via builder-like methods
    // ChatId can be username of a channel
    let request = api.send_message("@channelusername","Text").reply_to_message_id(42);

    // All requests implement IntoFuture
    let message = request.await?;

    // You can handle some common API errors automatically
    let message = API::request(request).await?;
```

## Mutating and calling requests by references
```rust, no_run    
    let mut request = api.send_message(chat_id, "Text");
    for i in 0..5 {
        request = request.chat_id(i);

        // &Request implements IntoFuture too
        let message = (&request).await?;
    }
```

## FAQ
**Q: Is it production-ready?**<br>
A: It may be, or may be not. The library is not thoroughly tested, some stuff may be broken.<br><br>
**Q: Are there any bots made with it?**<br>
A: I developed this library to be used by [@hintorbot](https://t.me/hintorbot), so at least this one. If you want to list your bot here, contact me in tg: [@encrypted_for](https://t.me/encrypted_for)
