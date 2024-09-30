use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub trait RequestSerializable {
    fn serialize(&self, destination_address: String) -> Result<String>;
}

#[derive(Serialize, Deserialize)]
pub struct UpdateStateRequest {}

#[derive(Serialize, Deserialize)]
pub struct InitializeRequest {
    pub seeds: Vec<u8>,
}
impl RequestSerializable for InitializeRequest {
    fn serialize(&self, destination_address: String) -> Result<String> {
        let params: [&dyn erased_serde::Serialize; 2] = [
            &destination_address,
            &json!({
                "data": self.seeds,
            }),
        ];
        let payload = serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "initialize",
            "params": params,
            "id": ic_cdk::api::time(),
        }))?;
        Ok(payload)
    }
}
