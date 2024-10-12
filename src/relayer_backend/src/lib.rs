use std::str::FromStr;

use anyhow::Result;
use ic_cdk::api::management_canister::http_request::{
    HttpResponse, TransformArgs, TransformContext, TransformFunc,
};
use serde_json::json;
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
async fn balance() -> u64 {
    devnet_client()
        .get_balance(&local_signer().await.pubkey(), call_opt())
        .await
        .unwrap()
}

#[ic_cdk::update]
async fn sample_airdrop() {
    let signer = local_signer().await;
    let client = devnet_client();
    let result = client
        .request_airdrop(&signer.pubkey(), LAMPORTS_PER_SOL, call_opt())
        .await;
    match result {
        Ok(tx) => ic_cdk::print(format!("Airdrop successful: {:?}", tx).as_str()),
        Err(e) => ic_cdk::print(format!("Airdrop failed: {:?}", e).as_str()),
    }
}

fn call_opt() -> CallOptions {
    let mut opt = CallOptions::default();
    opt.transform = Some(TransformContext {
        context: vec![],
        function: TransformFunc {
            0: candid::Func {
                principal: ic_cdk::api::id(),
                method: "transform".to_string(),
            },
        },
    });
    opt
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
async fn get_pubkey() -> String {
    local_signer().await.pubkey().to_string()
}

macro_rules! retry {
    ($f:expr, $count:expr) => {{
        let mut retries = 0;
        let result = loop {
            let result = $f;
            if result.is_ok() {
                break result;
            } else if retries > $count {
                break result;
            } else {
                retries += 1;
            }
        };
        result
    }};
    ($f:expr) => {
        retry!($f, 5)
    };
}

#[ic_cdk::update]
async fn sample_instruction() {
    let program = Pubkey::from_str(DESTINATION).unwrap();
    let signer = local_signer().await;
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
    let signers: Vec<Box<dyn Signer>> = vec![Box::new(signer.clone())];
    let block_hash = retry!(client.get_latest_blockhash(call_opt()).await);
    let tx = Transaction::new_signed_with_payer(
        &vec![instruction],
        Some(&signer.pubkey()),
        &signers,
        block_hash.unwrap(),
    )
    .await;
    let result = client.send_transaction(&tx, call_opt()).await;
    ic_cdk::println!("Transaction result: {:?}", result);
}

fn devnet_client() -> WasmClient {
    WasmClient::new("https://rpc.ankr.com/solana_devnet/86cce205fecf9f9fe4271a15e93e686be2ce71786a8c522f8a1da15a50e4aeac")
}

async fn local_signer() -> ThresholdSigner {
    ThresholdSigner::new(SchnorrKeyIds::ProductionKey1)
        .await
        .unwrap()
}
