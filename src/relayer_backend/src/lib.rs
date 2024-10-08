use solana_client_icp::{
    solana_sdk::signer::{
        threshold_signer::{SchnorrKeyIds, ThresholdSigner},
        Signer,
    },
    WasmClient,
};

thread_local! {
    static DESTINATION : std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
}
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
fn set_program_destination(destination: String) {
    DESTINATION.with(|d| {
        *d.borrow_mut() = Some(destination);
    });
}

#[ic_cdk::query]
fn get_program_destination() -> String {
    DESTINATION.with(|d| d.borrow().clone().unwrap_or_else(|| "".to_string()))
}

#[ic_cdk::update]
async fn sample() {
    let client = WasmClient::new("https://api.devnet.solana.com");
    let signer = ThresholdSigner::new(SchnorrKeyIds::TestKeyLocalDevelopment)
        .await
        .unwrap();
    let balance = client.get_balance(&signer.pubkey()).await.unwrap();
    ic_cdk::print(format!("Balance: {}", balance).as_str());
}
