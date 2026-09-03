use envconfig::Envconfig;

#[derive(Clone, Envconfig)]
pub struct AppConfig {
    #[envconfig(nested)]
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn init() -> Self {
        Self::init_from_env().expect("Failed to load configuration! Check the .env file.")
    }
}

#[derive(Clone, Envconfig)]
pub struct ServerConfig {
    #[envconfig(from = "SERVER_PORT")]
    pub port: u16,

    #[envconfig(from = "SERVER_ALLOWED_ORIGINS")]
    pub allowed_origin: String,

    #[envconfig(from = "SERVER_ALLOWED_METHODS")]
    pub allowed_methods: String,

    #[envconfig(from = "SERVER_ALLOWED_HEADERS")]
    pub allowed_headers: String,

    #[envconfig(from = "SERVER_DEFAULT_BODY_LIMIT", default = "104345")]
    pub default_body_limit: usize,
}

impl ServerConfig {
    pub fn to_addr(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
