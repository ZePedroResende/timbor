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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let opts = Opts::parse_args_default_or_exit();

    println!("[pending-stream]");

    let provider = Provider::<Ws>::connect(opts.url.as_str()).await?;
    let provider = Arc::new(provider);

    let last_block = provider
        .get_block(BlockNumber::Number(U64::from(13965258)))
        .await?
        .unwrap()
        .number
        .unwrap();

    let address: Address = "0x5738379364fab26c7e044c02ded4ceef93333d84".parse()?;
    let erc721_transfer_filter = Filter::new()
        .from_block(last_block)
        .address(address)
        .topic0(ValueOrArray::Value(H256::from(keccak256(
            "Transfer(address,address,uint256)",
        ))));

    let mut stream = provider.subscribe_logs(&erc721_transfer_filter).await?;

    while let Some(log) = stream.next().await {
        println!(
            "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}, id: {:?}",
            log.block_number,
            log.transaction_hash,
            log.address,
            Address::from(log.topics[1]),
            Address::from(log.topics[2]),
            Address::from(log.topics[3]),
        );
    }

    Ok(())
}
