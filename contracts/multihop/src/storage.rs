use crate::error::ContractError;
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Swap {
    pub ask_asset: Address,
    pub offer_asset: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct Pair {
    pub token_a: Address,
    pub token_b: Address,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    PairKey(Pair),
    Admin,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Asset {
    /// Address of the asset
    pub address: Address,
    /// The total amount of those tokens in the pool
    pub amount: i128,
}

/// This struct is used to return a query result with the total amount of LP tokens and assets in a specific pool.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PoolResponse {
    /// The asset A in the pool together with asset amounts
    pub asset_a: Asset,
    /// The asset B in the pool together with asset amounts
    pub asset_b: Asset,
    /// The total amount of LP tokens currently issued
    pub asset_lp_share: Asset,
}

pub fn save_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, &admin);
}

pub fn _get_admin(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(ContractError::AdminNotFound)
}

pub fn save_factory(env: &Env, pair: Pair, factory: Address) {
    env.storage()
        .instance()
        .set(&DataKey::PairKey(pair), &factory);
}

pub fn get_factory(env: &Env, pair: Pair) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::PairKey(pair))
        .ok_or(ContractError::FactoryNotFound)
}
