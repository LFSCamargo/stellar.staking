#![allow(unused)]
use soroban_sdk::{contracttype, symbol_short, Address, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct StakingContractState {
  pub staking_token: Address,
  pub reward_token: Address,
  pub owner: Address,
  pub initialized: bool,
}

pub const STAKING_STATE: Symbol = symbol_short!("state");

#[contracttype]
#[derive(Clone)]
pub struct StakingPoolInfo {
  pub duration: u64,
  pub finish_at: u64,
  pub updated_at: u64,
  pub reward_rate: u64,
  pub reward_per_token_stored: u64,
  pub total_supply: u64,
}

pub const STAKING_POOL_INFO: Symbol = symbol_short!("poolinfo");

#[contracttype]
pub enum UserInfoRegistry {
  UserRecord(Address),
}

#[contracttype]
#[derive(Clone)]
pub struct UserRecord {
  pub address: Address,
  pub staked_amount: u64,
  pub reward_amount: u64,

  pub created_at: u64,
  pub updated_at: u64,
}
