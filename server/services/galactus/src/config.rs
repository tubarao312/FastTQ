use dotenv::dotenv;

pub fn load_env() {
    // Load only in development
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
}
