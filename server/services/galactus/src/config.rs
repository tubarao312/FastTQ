use dotenv::dotenv;

pub struct Config {
    broker_addr: String,
}

fn load_env() {
    // Load only in development
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
}

impl Config {
    pub fn new() -> Config {
        load_env();

        Config {
            broker_addr: std::env::var("BROKER_ADDR").unwrap(),
        }
    }

    pub fn broker_addr(&self) -> &str {
        &self.broker_addr
    }
}
