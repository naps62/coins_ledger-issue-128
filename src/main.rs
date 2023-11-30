use ethers::signers::{HDPath, Ledger, LedgerError};

#[tokio::main]
async fn main() {
    // first call, works
    let _ = dbg!(call_ledger().await);

    // tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // second call, if done without sleep first, fails with `hid_error is not implemented yet`
    let _ = dbg!(call_ledger().await);
}

pub(crate) async fn call_ledger() -> Result<Ledger, LedgerError> {
    let path: String = "m/44'/60'/0'/0/0".into();
    Ledger::new(HDPath::Other(path), 1).await
}
