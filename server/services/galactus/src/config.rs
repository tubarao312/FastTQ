use dotenv::dotenv;

pub struct Config {
    pub broker_addr: String,
    pub db_reader_url: String,
    pub db_writer_url: String,
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
            broker_addr: std::env::var("FASTTQ_BROKER_ADDR").unwrap(),
            db_reader_url: std::env::var("DATABASE_READER_URL").unwrap(),
            db_writer_url: std::env::var("DATABASE_WRITER_URL").unwrap(),
        }
    }
}
