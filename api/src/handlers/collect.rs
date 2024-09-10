use crate::handlers::services;
use crate::types::{CollectRequest, ValuesType};
use axum::http::StatusCode;
use axum::response::Json;
use ethers::prelude::*;
use ethers::utils::Anvil;
use std::{convert::TryFrom, sync::Arc, time::Duration};

abigen!(Collect, "../contracts/out/Collect.sol/Collect.json");
abigen!(TestToken, "../contracts/out/TestToken.sol/TestToken.json");

pub async fn collect_eth_handler(Json(payload): Json<CollectRequest>) -> (StatusCode, String) {
    if payload.values.len() > 5 {
        return (StatusCode::BAD_REQUEST, "Too many values".to_string());
    }

    if payload.values.is_empty() {
        return (StatusCode::OK, "No values provided".to_string());
    }

    let total_amount = payload.total_amount;
    match payload.values_type {
        ValuesType::Amount => {
            return collect_eth(payload.values).await;
        }
        ValuesType::Percentage => {
            if total_amount.is_none() {
                return (
                    StatusCode::BAD_REQUEST,
                    "Total amount not provided".to_string(),
                );
            }

            let total_amount = total_amount.unwrap();
            let (status, result) =
                services::calculate_amounts_from_percentages(&payload.values, total_amount);

            match result {
                Ok(amounts) => return collect_eth(amounts).await,
                Err(message) => return (status, message),
            }
        }
    }
}

async fn collect_eth(amounts: Vec<u128>) -> (StatusCode, String) {
    let anvil = Anvil::new().spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into(); // client wallet and token receiver

    let provider = Provider::<Http>::try_from(anvil.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64));

    let client = Arc::new(SignerMiddleware::new(
        provider.clone(),
        wallet.clone().with_chain_id(anvil.chain_id()),
    ));

    let collect_contract = Collect::deploy(client, ()).unwrap().send().await.unwrap();

    collect_contract
        .create_withrawal_contracts()
        .send()
        .await
        .unwrap();
    let withdrawal_contracts = collect_contract
        .get_withdrawal_contracts()
        .call()
        .await
        .unwrap();

    for i in 0..amounts.len() {
        let tmp_client = Arc::new(SignerMiddleware::new(
            provider.clone(),
            wallet.clone().with_chain_id(anvil.chain_id()),
        ));
        let tx = TransactionRequest::new()
            .to(withdrawal_contracts[i])
            .value(1000);
        let pending_tx = tmp_client.send_transaction(tx, None).await.unwrap();
        pending_tx
            .await
            .unwrap()
            .ok_or_else(|| eyre::format_err!("tx dropped from mempool"))
            .unwrap();
    }

    let amounts_u256: Vec<U256> = amounts.iter().map(|&x| U256::from(x)).collect();

    let collect_eth_call = collect_contract.collect_eth(amounts_u256);
    let collect_eth_send = collect_eth_call.send().await;

    match collect_eth_send {
        Ok(tx_receipt) => {
            return (
                StatusCode::OK,
                format!("Transaction successful: {:?}", tx_receipt.tx_hash()),
            );
        }
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Transaction failed: {:?}", e),
            );
        }
    }
}

// Handler for /collect/erc20
pub async fn collect_erc20_handler(Json(payload): Json<CollectRequest>) -> (StatusCode, String) {
    if payload.values.len() > 2 {
        return (StatusCode::BAD_REQUEST, "Too many values".to_string());
    }

    if payload.values.is_empty() {
        return (StatusCode::OK, "No values provided".to_string());
    }

    let total_amount = payload.total_amount;
    match payload.values_type {
        ValuesType::Amount => {
            return collect_erc20(payload.values).await;
        }
        ValuesType::Percentage => {
            if total_amount.is_none() {
                return (
                    StatusCode::BAD_REQUEST,
                    "Total amount not provided".to_string(),
                );
            }

            let total_amount = total_amount.unwrap();
            let (status, result) =
                services::calculate_amounts_from_percentages(&payload.values, total_amount);

            match result {
                Ok(amounts) => return collect_erc20(amounts).await,
                Err(message) => return (status, message),
            }
        }
    }
}

async fn collect_erc20(amounts: Vec<u128>) -> (StatusCode, String) {
    let anvil = Anvil::new().spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let sender1: LocalWallet = anvil.keys()[1].clone().into();
    let sender2: LocalWallet = anvil.keys()[2].clone().into();
    let receiver: LocalWallet = anvil.keys()[3].clone().into();

    let provider = Provider::<Http>::try_from(anvil.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64));

    let client = Arc::new(SignerMiddleware::new(
        provider.clone(),
        wallet.clone().with_chain_id(anvil.chain_id()),
    ));
    let sender1_client = Arc::new(SignerMiddleware::new(
        provider.clone(),
        sender1.clone().with_chain_id(anvil.chain_id()),
    ));
    let sender2_client = Arc::new(SignerMiddleware::new(
        provider.clone(),
        sender2.clone().with_chain_id(anvil.chain_id()),
    ));

    let collect_contract = Collect::deploy(client.clone(), ())
        .unwrap()
        .send()
        .await
        .unwrap();
    let test_erc20_contract = TestToken::deploy(sender1_client.clone(), sender1.address())
        .unwrap()
        .send()
        .await
        .unwrap();

    test_erc20_contract
        .transfer(sender2.clone().address(), U256::from(1000000))
        .send()
        .await
        .unwrap();

    // to call contact from different addresses we need different clients and instances
    let test_contract_sender1 = TestToken::new(test_erc20_contract.address(), sender1_client);
    let test_contract_sender2 = TestToken::new(test_erc20_contract.address(), sender2_client);

    test_contract_sender1
        .approve(collect_contract.address().clone(), U256::from(1000000))
        .send()
        .await
        .unwrap();
    test_contract_sender2
        .approve(collect_contract.address().clone(), U256::from(1000000))
        .send()
        .await
        .unwrap();

    let amounts_u256: Vec<U256> = amounts.iter().map(|&x| U256::from(x)).collect();
    let collect_contract_call = collect_contract.collect_erc20(
        test_erc20_contract.address(),
        receiver.address(),
        vec![sender1.address(), sender2.address()],
        amounts_u256,
    );
    let collect_contract_send = collect_contract_call.send().await;

    match collect_contract_send {
        Ok(tx_receipt) => {
            return (
                StatusCode::OK,
                format!("Transaction successful: {:?}", tx_receipt.tx_hash()),
            );
        }
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Transaction failed: {:?}", e),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::{body::Body, routing::post, Router};
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_collect_eth_valid_amounts() {
        let app = Router::new().route("/collect/eth", post(collect_eth_handler));

        let payload = json!({
            "values": [100, 200, 300],
            "total_amount": 600,
            "values_type": "Amount"
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/collect/eth")
                    .header("Content-Type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_collect_eth_invalid_percentage() {
        let app = Router::new().route("/collect/eth", post(collect_eth_handler));

        let payload = json!({
            "values": [10, 20, 30],
            "total_amount": 1000,
            "values_type": "Percentage"
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/collect/eth")
                    .header("Content-Type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
