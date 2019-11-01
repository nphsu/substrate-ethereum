#[macro_use] 
extern crate log;
use dotenv::dotenv;
use env_logger;

mod config;
mod ethereum_event_handler;

fn main() {
    dotenv().ok();
    env_logger::init();

    let config = config::Config::load().expect("cannot load config");
    info!("Ethereum  API URL : {}", &config.eth_api_url);
    info!("Substrate API URL : {}", &config.sub_api_url);

    ethereum_event_handler::start(config);
}
