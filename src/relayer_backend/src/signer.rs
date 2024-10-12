use solana_client_icp::solana_sdk::signer::threshold_signer::{SchnorrKeyIds, ThresholdSigner};

pub async fn signer(key_ids: SchnorrKeyIds) -> ThresholdSigner {
    ThresholdSigner::new(key_ids).await.unwrap()
}
