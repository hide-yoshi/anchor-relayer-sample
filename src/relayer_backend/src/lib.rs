use std::str::FromStr;
mod signer;
mod tx;
use anyhow::Result;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use serde_json::json;
use signer::signer;
use solana_client_icp::{
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signer::{threshold_signer::SchnorrKeyIds, Signer},
        system_program,
    },
    WasmClient,
};
use tx::TransactionInvoker;
const DESTINATION: &str = "9z3hM1vW44tw2uPh99Jh59xzEHjVaRoNzj6Nymcuv86V";
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
async fn balance() -> u64 {
    let signer = signer(SchnorrKeyIds::ProductionKey1).await;
    TransactionInvoker::new(signer.clone(), devnet_client())
        .get_balance(signer.pubkey())
        .await
        .unwrap()
}

#[ic_cdk::update]
async fn airdrop() {
    let signer = signer(SchnorrKeyIds::ProductionKey1).await;
    TransactionInvoker::new(signer.clone(), devnet_client())
        .airdrop(signer.pubkey())
        .await
        .unwrap();
}

#[ic_cdk::query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let body: Result<serde_json::Value> =
        serde_json::from_slice(&raw.response.body).map_err(Into::into);
    if let Err(_) = body {
        return HttpResponse {
            status: raw.response.status,
            headers: vec![],
            body: raw.response.body,
        };
    }
    let mut body = body.unwrap();
    ic_cdk::println!("Transforming response: {:?}", body);

    *body.get_mut("context").unwrap_or(&mut json!({"slot": 0})) = json!({ "slot": 0 });

    HttpResponse {
        status: raw.response.status,
        headers: vec![],
        body: serde_json::to_vec(&body).unwrap(),
    }
}

#[ic_cdk::update]
async fn pubkey() -> String {
    signer(SchnorrKeyIds::ProductionKey1)
        .await
        .pubkey()
        .to_string()
}

#[ic_cdk::update]
async fn test_instruction() {
    let program = Pubkey::from_str(DESTINATION).unwrap();
    let signer = signer(SchnorrKeyIds::ProductionKey1).await;
    let client = devnet_client();
    let key = "TEST3".to_string();
    let key_utf8 = key.as_bytes().as_ref();
    let initialize_discriminant: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
    let pda = Pubkey::find_program_address(&vec![key_utf8], &program);
    let instruction = Instruction::new_with_borsh(
        program,
        &(initialize_discriminant, "TEST3".to_string()),
        vec![
            AccountMeta::new(pda.0, false),
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );
    TransactionInvoker::new(signer, client)
        .invoke_instruction(instruction)
        .await
        .unwrap();
}

fn devnet_client() -> WasmClient {
    let denv = include_str!("./.env");
    let rpc_url = denv
        .split("\n")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split("=")
        .last()
        .unwrap();

    WasmClient::new(rpc_url)
}
