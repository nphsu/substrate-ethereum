use crate::config;
use futures::{future::Future, stream::Stream};
use std::sync::Arc;
use tokio_threadpool::blocking;
use web3::*;

pub fn start(config: config::Config) {
    let (_eloop, transport) = web3::transports::WebSocket::new(&config.eth_api_url).unwrap();
    let web3 = web3::Web3::new(transport);
    let web3 = Arc::new(web3);

    let fut = web3
        .eth_subscribe()
        .subscribe_logs(web3::types::FilterBuilder::default().build())
        .then(move |sub| {
            sub.unwrap().for_each(move |log| {
                log::info!("[ethereum] got log: {:?}", log);
                Ok(())
            })
        })
        .map_err(|_| ());
    tokio::run(fut);
}
