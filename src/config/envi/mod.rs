use dotenv::dotenv;

pub fn init_env() {
    dotenv().ok();
}
