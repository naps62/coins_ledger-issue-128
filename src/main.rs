use ethers::signers::{HDPath, Ledger, LedgerError};

#[tokio::main]
async fn main() {
    // first call, works
    // if ledger is disconnected we get Err(UnexpectedNullResponse) as expected
    let _ = dbg!(call_ledger().await);

    // ISSUE 1
    // second call works if ledger is connected,
    // but panics if its disconnected
    // comment this to reach 2nd issue
    // let _ = dbg!(call_ledger().await);

    // ISSUE 2
    // these fail, first with Hid(InitializationError), then with a poisoned mutex
    // regardless of ledger being connected or not
    let _ = tokio::spawn(call_ledger()).await;
    let _ = tokio::spawn(call_ledger()).await;
}

pub(crate) async fn call_ledger() -> Result<Ledger, LedgerError> {
    let path: String = "m/44'/60'/0'/0/0".into();
    Ledger::new(HDPath::Other(path), 1).await
}
