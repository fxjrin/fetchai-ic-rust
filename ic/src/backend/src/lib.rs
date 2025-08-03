use ic_cdk::*;
use ic_http_certification::{HttpRequest, HttpResponse, HttpUpdateResponse};
use serde::{Deserialize, Serialize};

// ---------- Type definitions for API requests & responses ----------

// Request type for get_balance endpoint
#[derive(Deserialize)]
pub struct GetBalanceRequestJson {
    pub address: String,
}

// Response type for get_balance
#[derive(Serialize)]
pub struct GetBalanceResponse {
    pub address: String,
    pub balance: f64,
    pub unit: String,
}

// Request type for get_utxos
#[derive(Deserialize)]
pub struct GetUtxosRequestJson {
    pub address: String,
}

// Response type for get_utxos (single UTXO)
#[derive(Serialize)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
    pub confirmations: u32,
}

// Request type for send endpoint
#[derive(Deserialize)]
pub struct SendRequestJson {
    #[serde(rename = "destinationAddress")]
    pub destination_address: String,
    #[serde(rename = "amountInSatoshi")]
    pub amount_in_satoshi: u64,
}

// Response type for send
#[derive(Serialize)]
pub struct SendResponse {
    pub success: bool,
    pub destination: String,
    pub amount: u64,
    #[serde(rename = "txId")]
    pub tx_id: String,
}

// Response type for welcome message
#[derive(Serialize)]
pub struct WelcomeMessage {
    pub message: String,
}

// Response type for get_p2pkh_address
#[derive(Serialize)]
pub struct GetP2pkhAddressResponse {
    pub address: String,
}

// Response type for dummy_test
#[derive(Serialize)]
pub struct DummyTestResponse {
    pub status: String,
    pub data: DummyTestData,
}

#[derive(Serialize)]
pub struct DummyTestData {
    pub message: String,
    pub timestamp: String,
    #[serde(rename = "testData")]
    pub test_data: TestData,
}

#[derive(Serialize)]
pub struct TestData {
    pub id: u32,
    pub name: String,
    pub value: f64,
    #[serde(rename = "isTest")]
    pub is_test: bool,
}

// ---------- HTTP query & update entry points ----------

// Handles GET / OPTIONS and returns an upgradeable response
#[query]
fn http_request(_req: HttpRequest) -> HttpResponse<'static> {
    HttpResponse::builder().with_upgrade(true).build()
}

// Handles POST routes and routes to specific handlers
#[update]
fn http_request_update(req: HttpRequest) -> HttpUpdateResponse<'static> {
    let url = req.url();
    
    // Simple routing based on URL content
    if url.contains("/get-balance") { //
        handle_get_balance(req)
    } else if url.contains("/get-utxos") { //
        handle_get_utxos(req)
    } else if url.contains("/get-current-fee-percentiles") { //
        handle_get_fee_percentiles()
    } else if url.contains("/get-p2pkh-address") {
        handle_get_p2pkh_address()
    } else if url.contains("/send") { //
        handle_send(req)
    } else if url.contains("/dummy-test") {
        handle_dummy_test()
    } else {
        handle_welcome()
    }
}

// ---------- API handlers ----------

// Welcome message
fn handle_welcome() -> HttpUpdateResponse<'static> {
    let welcome = WelcomeMessage {
        message: "Welcome to the Dummy Bitcoin Canister API".to_string(),
    };
    json_response(&welcome)
}

// Dummy: Returns the balance of a given Bitcoin address
fn handle_get_balance(req: HttpRequest) -> HttpUpdateResponse<'static> {
    match serde_json::from_slice::<GetBalanceRequestJson>(req.body()) {
        Ok(request) => {
            let response = GetBalanceResponse {
                address: request.address,
                balance: 0.005,
                unit: "BTC".to_string(),
            };
            json_response(&response)
        }
        Err(_) => error_response("Invalid request body")
    }
}

// Dummy: Returns the UTXOs of a given Bitcoin address
fn handle_get_utxos(req: HttpRequest) -> HttpUpdateResponse<'static> {
    match serde_json::from_slice::<GetUtxosRequestJson>(req.body()) {
        Ok(_request) => {
            let utxos = vec![
                Utxo {
                    txid: "dummy-txid-1".to_string(),
                    vout: 0,
                    value: 25000,
                    confirmations: 5,
                },
                Utxo {
                    txid: "dummy-txid-2".to_string(),
                    vout: 1,
                    value: 50000,
                    confirmations: 3,
                },
            ];
            json_response(&utxos)
        }
        Err(_) => error_response("Invalid request body")
    }
}

// Dummy: Returns the 100 fee percentiles measured in millisatoshi/byte
fn handle_get_fee_percentiles() -> HttpUpdateResponse<'static> {
    let fees: Vec<u64> = (100..200).collect();
    json_response(&fees)
}

// Dummy: Returns the P2PKH address of this canister
fn handle_get_p2pkh_address() -> HttpUpdateResponse<'static> {
    let response = GetP2pkhAddressResponse {
        address: "tb1qdummyaddressxyz1234567890".to_string(),
    };
    json_response(&response)
}

// Dummy: Sends satoshis from this canister to a specified address
fn handle_send(req: HttpRequest) -> HttpUpdateResponse<'static> {
    match serde_json::from_slice::<SendRequestJson>(req.body()) {
        Ok(request) => {
            let response = SendResponse {
                success: true,
                destination: request.destination_address,
                amount: request.amount_in_satoshi,
                tx_id: "dummy-txid-sent-1234567890".to_string(),
            };
            json_response(&response)
        }
        Err(_) => error_response("Invalid request body")
    }
}

// Dummy test endpoint
fn handle_dummy_test() -> HttpUpdateResponse<'static> {
    let response = DummyTestResponse {
        status: "success".to_string(),
        data: DummyTestData {
            message: "This is a dummy response".to_string(),
            timestamp: "2024-01-01T12:00:00.000Z".to_string(),
            test_data: TestData {
                id: 1,
                name: "Test Bitcoin Data".to_string(),
                value: 0.001,
                is_test: true,
            },
        },
    };
    json_response(&response)
}

// ---------- Helper functions ----------

// Builds a JSON response from any serializable type
fn json_response<T: Serialize>(data: &T) -> HttpUpdateResponse<'static> {
    HttpResponse::builder()
        .with_body(serde_json::to_vec(data).unwrap_or_else(|_| b"{}".to_vec()))
        .build_update()
}

// Builds an error JSON response
fn error_response(message: &str) -> HttpUpdateResponse<'static> {
    HttpResponse::builder()
        .with_body(format!(r#"{{"error":"{}"}}"#, message).into_bytes())
        .build_update()
}