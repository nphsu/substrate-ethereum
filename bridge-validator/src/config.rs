use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub eth_api_url: String,
    pub sub_api_url: String,
}

impl Config {
    pub fn load() -> Result<Self, &'static str> {
        Ok(Config {
            eth_api_url: parse_eth_api_url()?,
            sub_api_url: parse_sub_api_url()?,
        })
    }
}

fn parse_eth_api_url() -> Result<String, &'static str> {
    env::var("ETH_API_URL").map_err(|_| "cannot read ETH_API_URL")
}

fn parse_sub_api_url() -> Result<String, &'static str> {
    env::var("SUB_API_URL").map_err(|_| "cannot read SUB_API_URL")
}