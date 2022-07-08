mod pendingStream;
use ethers::contract::{Contract, EthEvent};
use std::process::Command;
//use ethers::core::types::{Address, Filter, ValueOrArray, H256, U256};
use ethers::abi::AbiDecode;
use ethers::prelude::*;
use ethers::providers::Authorization;
use ethers::providers::{Provider, Ws};
use ethers::utils::keccak256;
use gumdrop::Options;
use notify_rust::Notification;
use pendingStream::pending_stream;
use std::sync::Arc;

#[derive(Debug, Options, Clone)]
struct Opts {
    help: bool,

    #[options(default = "ws://localhost:8546", help = "Node Websocket URL")]
    url: String,
    username: String,
    password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let opts = Opts::parse_args_default_or_exit();

    let provider = Provider::<Ws>::connect_with_auth(
        opts.url,
        Authorization::basic(opts.username, opts.password),
    )
    .await?;

    let provider = Arc::new(provider);

    let last_block = provider
        .get_block(BlockNumber::Number(U64::from(14896795)))
        .await?
        .unwrap()
        .number
        .unwrap();

    let address: Address = "0xbdc105c068715d57860702da9fa0c5ead11fba51".parse()?;
    let erc721_transfer_filter = Filter::new()
        .from_block(last_block)
        .address(address)
        .topic0(ValueOrArray::Value(H256::from(keccak256(
            "Transfer(address,address,uint256)",
        ))));

    let mut stream = provider.subscribe_logs(&erc721_transfer_filter).await?;

    while let Some(log) = stream.next().await {
        //        println!(
        //            "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}, id: {:?}",
        //            log.block_number,
        //            log.transaction_hash,
        //            log.address,
        //            Address::from(log.topics[1]),
        //            Address::from(log.topics[2]),
        //            Address::from(log.topics[3]),
        //        );
        println!("block: id: {:?}", U256::decode(log.topics[3]));
        let id = U256::decode(log.topics[3])?;
        let address = "0x59A9fFfEc1C84DF01Cdc3323008DF324594877e6".parse()?;
        if Address::from(log.topics[2]) == address {
            Notification::new()
                .sound_name("message-new-instant")
                .summary("NOGraph")
                .body("O teu foi minted !!!!!")
                .show()?;
        } else {
            let s: String = id.to_string().to_owned();
            let st: String = format!("NOGraph {}", &s[..]);
            Notification::new().summary(&st[..]).body(&st[..]).show()?;
        }
    }

    Ok(())
}
