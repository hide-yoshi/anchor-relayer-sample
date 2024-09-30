use ic_cdk::{
    api::management_canister::http_request::{HttpResponse, TransformArgs},
    query,
};

mod rpc;

thread_local! {
    static SOL_DESTINATION_ADDRESS: std::cell::RefCell<String> = std::cell::RefCell::new("".to_string());
}

#[ic_cdk::update]
fn update_sol_destination_address(addres: String) {
    SOL_DESTINATION_ADDRESS.with_borrow_mut(|cell| {
        *cell = addres;
    });
}

#[ic_cdk::query]
fn get_sol_destination_address() -> String {
    SOL_DESTINATION_ADDRESS.with_borrow(|cell| cell.clone())
}

#[ic_cdk::update]
async fn initialize(token: String) -> String {
    let destination_address = get_sol_destination_address();
    let client = rpc::SolRpcClient::new("https://api.devnet.solana.com".to_string());
    let res = client
        .initialize(token.as_bytes().to_vec(), destination_address)
        .await;
    match res {
        Ok(res) => res,
        Err(e) => format!("Error: {:?}", e),
    }
}

/// Cleans up the HTTP response headers to make them deterministic.
///
/// # Arguments
///
/// * `args` - Transformation arguments containing the HTTP response.
#[query(hidden = true)]
fn cleanup_response(mut args: TransformArgs) -> HttpResponse {
    // The response header contain non-deterministic fields that make it impossible to reach consensus!
    // Errors seem deterministic and do not contain data that can break consensus.

    // Clear non-deterministic fields from the response headers.
    args.response.headers.clear();

    args.response
}

ic_cdk_macros::export_candid!();
