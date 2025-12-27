use async_graphql::SimpleObject;
use uuid::Uuid;

#[derive(SimpleObject)]
pub struct Stake {
    pub id: Uuid,
    pub user_id: Uuid,
    pub todo_id: Uuid,
    pub amount_lamports: u64,
    pub status: String,
    pub tx_signature: String,
}
