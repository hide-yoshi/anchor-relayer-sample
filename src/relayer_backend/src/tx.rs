use ic_cdk::api::management_canister::http_request::{TransformContext, TransformFunc};
use solana_client_icp::{
    solana_sdk::{
        hash::Hash, instruction::Instruction, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
        signature::Signature, signer::Signer, transaction::Transaction,
    },
    CallOptions, ClientError, WasmClient,
};
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

async fn new_tx<S: Signer>(instruction: Instruction, signer: S, block_hash: Hash) -> Transaction {
    let signers = vec![&signer];
    Transaction::new_signed_with_payer(
        &vec![instruction],
        Some(&signer.pubkey()),
        &signers,
        block_hash,
    )
    .await
}

pub struct TransactionInvoker<S: Signer> {
    pub signer: S,
    pub client: WasmClient,
}

impl<S: Signer + Clone> TransactionInvoker<S> {
    pub fn new(signer: S, client: WasmClient) -> Self {
        Self { signer, client }
    }
    pub async fn new_transaction(&self, instruction: Instruction, block_hash: Hash) -> Transaction {
        new_tx(instruction, self.signer.clone(), block_hash).await
    }
    pub async fn invoke_instruction(
        &self,
        instruction: Instruction,
    ) -> Result<Signature, ClientError> {
        let block_hash = retry!(self.client.get_latest_blockhash(call_opt()).await)?;
        let tx = self.new_transaction(instruction, block_hash).await;
        self.client.send_transaction(&tx, call_opt()).await
    }
    pub async fn get_balance(&self, pubkey: Pubkey) -> Result<u64, ClientError> {
        let res = self
            .client
            .get_balance_with_commitment(&pubkey, self.client.commitment_config(), call_opt())
            .await?;
        Ok(res)
    }
    pub async fn airdrop(&self, pubkey: Pubkey) -> Result<Signature, ClientError> {
        self.client
            .request_airdrop(&pubkey, LAMPORTS_PER_SOL, call_opt())
            .await
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
