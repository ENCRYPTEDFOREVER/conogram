#[derive(Clone)]
/// See [Using bot in test environment](https://core.telegram.org/bots/webapps#using-bots-in-the-test-environment)
/// See [Using local api server](https://core.telegram.org/bots/api#using-a-local-bot-api-server)
pub struct ApiServerConfig {
    pub url: String,
    pub port: u16,
    pub use_test_env: bool,
}

impl ApiServerConfig {
    pub fn new(url: String, port: u16, use_test_env: bool) -> Self {
        Self {
            url,
            port,
            use_test_env,
        }
    }

    pub fn remote(use_test_env: bool) -> Self {
        Self {
            url: "https://api.telegram.org".to_string(),
            port: 443,
            use_test_env,
        }
    }

    pub fn local(url: Option<String>, port: Option<u16>, use_test_env: bool) -> Self {
        Self {
            url: url.unwrap_or("http://localhost".to_string()),
            port: port.unwrap_or(80),
            use_test_env,
        }
    }
}

impl Default for ApiServerConfig {
    fn default() -> Self {
        ApiServerConfig::remote(false)
    }
}
