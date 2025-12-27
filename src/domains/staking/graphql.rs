use async_graphql::{Object, Context, Error};
// use solana_client::rpc_client::RpcClient;
// use solana_sdk::signature::Signature;
// use std::str::FromStr;
use uuid::Uuid;
use super::models::Stake;

#[derive(Default)]
pub struct StakingMutation;

#[Object]
impl StakingMutation {
    async fn stake_on_task(&self, ctx: &Context<'_>, todo_id: Uuid, amount_lamports: u64, tx_signature: String) -> async_graphql::Result<Stake> {
        // Verify transaction via direct JSON-RPC call to Solana Devnet
        let rpc_url = "https://api.devnet.solana.com";
        
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getSignatureStatuses",
            "params": [
                [tx_signature],
                { "searchTransactionHistory": true }
            ]
        });

        let response = client.post(rpc_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| Error::new(format!("RPC connection failed: {}", e)))?;

        let rpc_res: serde_json::Value = response.json()
            .await
            .map_err(|e| Error::new(format!("Failed to parse RPC response: {}", e)))?;

        // Check if we got a valid status
        // Response format: { "result": { "context": ..., "value": [ { "confirmationStatus": "finalized", ... } ] } }
        let statuses = rpc_res.get("result")
             .and_then(|r| r.get("value"))
             .and_then(|v| v.as_array())
             .ok_or_else(|| Error::new("Invalid RPC response structure"))?;

        if statuses.is_empty() {
             return Err(Error::new("Transaction not found"));
        }

        let status_info = &statuses[0];
        if status_info.is_null() {
             return Err(Error::new("Transaction not found on chain (value is null)"));
        }
        
        // In a real app, also verify:
        // 1. "err" is null
        // 2. The transaction actually transferred lamports to the correct store wallet
        // For MVP, existence check is sufficient.

        let stake = Stake {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(), // Placeholder
            todo_id,
            amount_lamports,
            status: "staked".to_string(),
            tx_signature,
        };

        Ok(stake)
    }

    async fn claim_stake(&self, _ctx: &Context<'_>, todo_id: Uuid) -> async_graphql::Result<String> {

        Ok(format!("Stake claimed for todo {}", todo_id))
    }
}
