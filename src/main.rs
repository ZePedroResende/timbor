mod pendingStream;
use ethers::contract::{Contract, EthEvent};
//use ethers::core::types::{Address, Filter, ValueOrArray, H256, U256};
use ethers::abi::AbiDecode;
use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use ethers::utils::keccak256;
use gumdrop::Options;
use pendingStream::pending_stream;
use std::sync::Arc;

#[derive(Debug, Options, Clone)]
struct Opts {
    help: bool,

    #[options(default = "ws://localhost:8546", help = "Node Websocket URL")]
    url: String,
}

#[derive(Clone, Debug, EthEvent)]
struct Transfer {
    from: Address,
    to: Address,
    id: U256,
}
fn parse_address(addr: &str) -> Address {
    let addr = addr.strip_prefix("0x").unwrap_or(addr);
    addr.parse().unwrap()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let opts = Opts::parse_args_default_or_exit();

    println!("[pending-stream]");

    let provider = Provider::<Ws>::connect(opts.url.as_str()).await?;
    let provider = Arc::new(provider);

    let last_block = provider
        .get_block(BlockNumber::Number(U64::from(14029156)))
        .await?
        .unwrap()
        .number
        .unwrap();

    let erc20_transfer_filter = Filter::new()
        .from_block(last_block)
        .topic0(ValueOrArray::Value(H256::from(keccak256(
            "Transfer(address,address,uint256)",
        ))));

    let mut stream = provider.get_logs_paginated(&erc20_transfer_filter, 100);

    while let Some(res) = stream.next().await {
        let log = res?;

        let address = parse_address("0x5738379364Fab26c7e044c02deD4ceef93333D84");
        if log.address == address {
            println!("{:?}", log);
        }
    }

    Ok(())
}
