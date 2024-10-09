use std::str::FromStr;

use solana_client_icp::{
    solana_sdk::{
        instruction::Instruction,
        native_token::LAMPORTS_PER_SOL,
        signer::{
            threshold_signer::{SchnorrKeyIds, ThresholdSigner},
            Signer,
        },
        transaction::Transaction,
    },
    CallOptions, WasmClient,
};
use solana_program::{instruction::AccountMeta, pubkey::Pubkey, system_program};
const DESTINATION: &str = "9z3hM1vW44tw2uPh99Jh59xzEHjVaRoNzj6Nymcuv86V";
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
async fn sample() {
    let balance = devnet_client()
        .get_balance(&local_signer().await.pubkey(), CallOptions::default())
        .await
        .unwrap();
    ic_cdk::print(format!("Balance: {}", balance).as_str());
}

#[ic_cdk::update]
async fn sample_airdrop() {
    let signer = local_signer().await;
    let client = devnet_client();
    let before_balance = client
        .get_balance(&signer.pubkey(), CallOptions::default())
        .await
        .unwrap();
    let result = client
        .request_airdrop(&signer.pubkey(), LAMPORTS_PER_SOL, CallOptions::default())
        .await;
    match result {
        Ok(tx) => ic_cdk::print(format!("Airdrop successful: {:?}", tx).as_str()),
        Err(e) => ic_cdk::print(format!("Airdrop failed: {:?}", e).as_str()),
    }
    let after_balance = client
        .get_balance(&signer.pubkey(), CallOptions::default())
        .await
        .unwrap();
    ic_cdk::println!(
        "Balance before: {}, after: {}",
        before_balance,
        after_balance
    );
}

#[ic_cdk::update]
async fn get_pubkey() -> String {
    local_signer().await.pubkey().to_string()
}

#[ic_cdk::update]
async fn sample_instruction() {
    let program = Pubkey::from_str(DESTINATION).unwrap();
    let signer = local_signer().await;
    let client = devnet_client();
    let key = "TEST2".to_string();
    let key_utf8 = key.as_bytes().as_ref();
    let initialize_discriminant: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
    let pda = Pubkey::find_program_address(&vec![key_utf8], &program);
    let instruction = Instruction::new_with_borsh(
        program,
        &(initialize_discriminant, "TEST2".to_string()),
        vec![
            AccountMeta::new(pda.0, false),
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );
    let signers: Vec<Box<dyn Signer>> = vec![Box::new(signer.clone())];
    let block_hash = client
        .get_latest_blockhash(CallOptions::default())
        .await
        .unwrap();
    let tx = Transaction::new_signed_with_payer(
        &vec![instruction],
        Some(&signer.pubkey()),
        &signers,
        block_hash,
    )
    .await;
    let result = client.send_transaction(&tx, CallOptions::default()).await;
    ic_cdk::println!("Transaction result: {:?}", result);
}

fn devnet_client() -> WasmClient {
    WasmClient::new("https://api.devnet.solana.com")
}

async fn local_signer() -> ThresholdSigner {
    ThresholdSigner::new(SchnorrKeyIds::TestKeyLocalDevelopment)
        .await
        .unwrap()
}
