#[macro_use] 
extern crate log;
use dotenv::dotenv;
use env_logger;
mod config;

fn main() {
    dotenv().ok();
    env_logger::init();

    let config_result = config::Config::load().expect("cannot load config");
    info!("Ethereum  API URL : {}", &config_result.eth_api_url);
    info!("Substrate API URL : {}", &config_result.sub_api_url);
}
