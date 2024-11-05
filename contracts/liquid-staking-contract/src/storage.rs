#![allow(unused)]
use soroban_sdk::{contracttype, symbol_short, Address, Env, Symbol};

use crate::errors;

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

pub struct StorageClient;

impl StorageClient {
  pub fn get_default_state(env: Env) -> StakingContractState {
    StakingContractState {
      staking_token: env.current_contract_address().clone(),
      reward_token: env.current_contract_address().clone(),
      owner: env.current_contract_address().clone(),
      initialized: false,
    }
  }

  pub fn get_default_user(env: Env, user: Address) -> UserRecord {
    UserRecord {
      staked_amount: 0,
      created_at: 0,
      reward_amount: 0,
      updated_at: 0,
      address: user.clone(),
    }
  }
}
