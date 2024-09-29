pub mod request;
use ic_cdk::api::management_canister::http_request::{CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext};
use ic_cdk::api::management_canister::http_request::http_request;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolRpcClient {
    url: String,
}
impl SolRpcClient {
    pub fn new(url: String) -> Self {
        Self { url }
    }
    async fn call(&self, payload: &String, effective_size_estimate: u64) -> anyhow::Result<String>{
        // Details of the values used in the following lines can be found here:
        // https://internetcomputer.org/docs/current/developer-docs/production/computation-and-storage-costs
        let base_cycles = 400_000_000u128 + 100_000u128 * (2 * effective_size_estimate as u128);

        const BASE_SUBNET_SIZE: u128 = 13;
        const SUBNET_SIZE: u128 = 34;
        let cycles = base_cycles * SUBNET_SIZE / BASE_SUBNET_SIZE;
        let request = CanisterHttpRequestArgument {
            url: self.url.to_string(),
            max_response_bytes: Some(effective_size_estimate),
            method: HttpMethod::POST,
            headers: vec![HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            }],
            body: Some(payload.as_bytes().to_vec()),
            transform: Some(TransformContext::from_name(
                "cleanup_response".to_owned(),
                vec![],
            )),
        };
       let res = http_request(request, cycles).await;
         match res {
              Ok(response) => {
                Ok(String::from_utf8(response.0.body).unwrap())
              }
              Err(err) => {
                Err(anyhow::anyhow!("Error: {:?}", err))
              }
         }
    }
}