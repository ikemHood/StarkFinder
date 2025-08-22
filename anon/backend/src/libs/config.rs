use std::env;

pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        // Load .env if present
        let _ = dotenvy::dotenv();

        let port = env::var("PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(8080);

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        Self { host, port }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
