use ic_cdk::{api::management_canister::http_request::{HttpResponse, TransformArgs}, query};

mod rpc;
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
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
