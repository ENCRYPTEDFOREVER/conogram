use conogram::{
    api::{APIConfig, API},
    entities::update::AllowedUpdates,
};

fn main() {
    let bot_token = "123456:AABBCCDDEEFF";
    let api_config = APIConfig::remote(bot_token, false);

    let api = API::new(api_config);

    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(err) => {
            println!("Error creating tokio runtime: {err}");
            return;
        }
    };

    match rt.block_on(run_bot(api)) {
        Ok(_) => {}
        Err(err) => println!("Error running bot: {err}"),
    }
}

async fn run_bot(mut api: API) -> Result<(), conogram::errors::Error> {
    api.set_allowed_updates(vec![AllowedUpdates::Message]);

    loop {
        for update in api.poll_once().await? {
            if let Some(message) = update.message {
                message.copy_to(&api, message.chat.id).await?;
            }
        }
    }
}
