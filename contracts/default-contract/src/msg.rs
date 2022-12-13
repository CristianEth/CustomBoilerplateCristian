use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::State;

#[cw_serde]
pub struct InstantiateMsg { 
    pub default_instantiate: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    DefaultExecute {}
}

#[cw_serde]
pub enum QueryMsg {
    Query1 {},
}

#[cw_serde]
pub enum MigrateMsg {}

