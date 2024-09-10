use crate::handlers::services;
use crate::handlers::services::{generate_addresses, get_solidity_address, sum_u256_vector};
use crate::types::{DisperseRequest, ValuesType};
use axum::http::StatusCode;
use axum::response::Json as AxumJson;
use ethers::prelude::*;
use ethers::utils::Anvil;
use std::{convert::TryFrom, sync::Arc, time::Duration};

abigen!(Disperse, "../contracts/out/Disperse.sol/Disperse.json");
abigen!(TestToken, "../contracts/out/TestToken.sol/TestToken.json");

// Handler for /disperse/eth
pub async fn disperse_eth_handler(
    AxumJson(payload): AxumJson<DisperseRequest>,
) -> (StatusCode, String) {
    if payload.values.len() > 100 {
        return (StatusCode::BAD_REQUEST, "Too many values".to_string());
    }

    if payload.values.is_empty() {
        return (StatusCode::OK, "No values provided".to_string());
    }

    let total_amount = payload.total_amount;
    match payload.values_type {
        ValuesType::Amount => {
            return disperse_eth(payload.values).await;
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
                Ok(amounts) => return disperse_eth(amounts).await,
                Err(message) => return (status, message),
            }
        }
    }
}

async fn disperse_eth(amounts: Vec<u128>) -> (StatusCode, String) {
    let anvil = Anvil::new().spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into(); // client wallet and token receiver

    let provider = Provider::<Http>::try_from(anvil.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64));

    let client = Arc::new(SignerMiddleware::new(
        provider.clone(),
        wallet.clone().with_chain_id(anvil.chain_id()),
    ));

    let disperse_contract = Disperse::deploy(client, ()).unwrap().send().await.unwrap();

    let disperce_addresses = generate_addresses(amounts.len() as u8).unwrap();
    let amounts_u256: Vec<U256> = amounts.iter().map(|&x| U256::from(x)).collect();

    let disperse_data: Vec<TransferData> = disperce_addresses
        .into_iter()
        .zip(amounts_u256.clone().into_iter())
        .map(|(wallet, amount)| TransferData {
            wallet: get_solidity_address(wallet),
            amount,
        })
        .collect();

    let collect_eth_call = disperse_contract
        .disperse_eth(disperse_data)
        .value(sum_u256_vector(amounts_u256));
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

// Handler for /disperse/erc20
pub async fn disperse_erc20_handler(
    AxumJson(payload): AxumJson<DisperseRequest>,
) -> (StatusCode, String) {
    if payload.values.len() > 100 {
        return (StatusCode::BAD_REQUEST, "Too many values".to_string());
    }

    if payload.values.is_empty() {
        return (StatusCode::OK, "No values provided".to_string());
    }

    let total_amount = payload.total_amount;
    match payload.values_type {
        ValuesType::Amount => {
            return disperse_erc20(payload.values).await;
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
                Ok(amounts) => return disperse_erc20(amounts).await,
                Err(message) => return (status, message),
            }
        }
    }
}

async fn disperse_erc20(amounts: Vec<u128>) -> (StatusCode, String) {
    let anvil = Anvil::new().spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into(); // client account and sender

    let provider = Provider::<Http>::try_from(anvil.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64));

    let client = Arc::new(SignerMiddleware::new(
        provider.clone(),
        wallet.clone().with_chain_id(anvil.chain_id()),
    ));

    let disperse_contract = Disperse::deploy(client.clone(), ())
        .unwrap()
        .send()
        .await
        .unwrap();
    let test_erc20_contract = TestToken::deploy(client.clone(), wallet.address())
        .unwrap()
        .send()
        .await
        .unwrap();

    test_erc20_contract
        .approve(disperse_contract.address().clone(), U256::from(1000000))
        .send()
        .await
        .unwrap();

    let disperce_addresses = generate_addresses(amounts.len() as u8).unwrap();
    let amounts_u256: Vec<U256> = amounts.iter().map(|&x| U256::from(x)).collect();

    let disperse_data: Vec<TransferData> = disperce_addresses
        .into_iter()
        .zip(amounts_u256.clone().into_iter())
        .map(|(wallet, amount)| TransferData {
            wallet: get_solidity_address(wallet),
            amount,
        })
        .collect();

    let disperse_contract_call = disperse_contract.disperse_erc20(
        test_erc20_contract.address(),
        wallet.address(),
        disperse_data,
    );
    let disperse_contract_send = disperse_contract_call.send().await;

    match disperse_contract_send {
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
        let app = Router::new().route("/disperse/eth", post(disperse_eth_handler));

        let payload = json!({
            "values": [100, 300],
            "values_type": "Amount"
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/disperse/eth")
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
        let app = Router::new().route("/disperse/eth", post(disperse_eth_handler));

        let payload = json!({
            "values": [10, 30],
            "total_amount": 1000,
            "values_type": "Percentage"
        });

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/disperse/eth")
                    .header("Content-Type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
