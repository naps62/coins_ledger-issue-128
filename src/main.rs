use std::sync::Arc;

use ethers::signers::HDPath;
use ethers::types::Address;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // first call, works
    let _ = dbg!(call_ledger().await);

    let m = Arc::new(Mutex::new(()));
    let handles = (0..3).map(|i| {
        let m = m.clone();

        tokio::spawn(async move {
            // even with a mutex to ensure only one call at a time, this poisons the HID mutex
            let guard = m.lock().await;
            println!("start {}", i);
            let _ = call_ledger().await;
            println!("end {}", i);
        })
    });

    for handle in handles {
        let _ = handle.await;
    }
}

pub(crate) async fn call_ledger() -> anyhow::Result<Address> {
    let path: String = "m/44'/60'/0'/0/0".into();
    let ledger = ethers::signers::Ledger::new(HDPath::Other(path), 1).await?;

    Ok(ledger.get_address().await?)
}
