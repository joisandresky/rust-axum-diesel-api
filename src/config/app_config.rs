use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct AppConfig {
    #[envconfig(from = "APP_PORT", default = "3001")]
    app_port: u16,

    #[envconfig(from = "DATABASE_URL")]
    database_url: String,

    // #[envconfig(from = "RUST_LOG", default = "info")]
    // log_level: String,
}

impl AppConfig {
    pub fn app_port(&self) -> u16 {
        self.app_port
    }
    
    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}