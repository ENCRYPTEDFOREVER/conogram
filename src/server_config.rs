#[derive(Debug, Clone)]
/// See [Using bot in test environment](https://core.telegram.org/bots/webapps#using-bots-in-the-test-environment)
/// See [Using local api server](https://core.telegram.org/bots/api#using-a-local-bot-api-server)
pub struct ApiServerConfig {
    pub url: String,
    pub use_test_env: bool,
}

impl ApiServerConfig {
    #[must_use]
    pub const fn new(url: String, use_test_env: bool) -> Self {
        Self { url, use_test_env }
    }

    #[must_use]
    pub fn remote(use_test_env: bool) -> Self {
        Self {
            url: "https://api.telegram.org".to_string(),
            use_test_env,
        }
    }

    #[must_use]
    pub fn local(url: Option<String>, use_test_env: bool) -> Self {
        Self {
            url: url.unwrap_or_else(|| "http://localhost".to_string()),
            use_test_env,
        }
    }
}

impl Default for ApiServerConfig {
    fn default() -> Self {
        Self::remote(false)
    }
}
