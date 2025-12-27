use async_graphql::MergedObject;

use crate::domains::auth::graphql::AuthMutation;
use crate::domains::todo::graphql::{TodoQuery, TodoMutation};
use crate::domains::staking::graphql::StakingMutation;

#[derive(MergedObject, Default)]
pub struct QueryRoot(pub TodoQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(pub AuthMutation, pub TodoMutation, pub StakingMutation);