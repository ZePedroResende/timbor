use ethers::providers::{Middleware, Provider, StreamExt, Ws};
use std::ops::Not;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

pub async fn pending_stream(provider: Arc<Provider<Ws>>) -> anyhow::Result<()> {
    let mut watcher = provider.subscribe_pending_txs().await?;
    let count = Arc::new(AtomicI32::new(0));

    while let Some(hash) = watcher.next().await {
        let provider = Arc::clone(&provider);

        let count = Arc::clone(&count);

        tokio::spawn(async move {
            let tx = provider.get_transaction(hash).await.unwrap();
            let number = count.fetch_add(1, Ordering::SeqCst);
            if tx.is_none().not() {
                println!("{} {:?}", number, hash);
            }
        });
    }
    Ok(())
}
