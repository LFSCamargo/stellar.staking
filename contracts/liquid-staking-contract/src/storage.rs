#![allow(unused)]
use soroban_sdk::{contracttype, symbol_short, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct StakingGlobals {
  pub token_supply: i128,            // token supply
  pub reward_per_token_stored: i128, // reward per token
  pub last_updated_time: i128,       // last updated time
}

pub const STAKING_GLOBALS: Symbol = symbol_short!("globals");

#[contracttype]
#[derive(Clone)]
pub struct StakingContractState {
  pub reward_token: Address, // s_rewardToken - is the token that will be sent to users as rewards
  pub base_token: Address,   // s_baseToken - is the token that will be locked for staking
  pub share_token: Address,  // s_shareToken - is the token that will be sent to staking contract
  pub owner: Address,        // s_owner - is the owner of the contract, that can change it's state
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
  pub address: Address,             // s_address - user's address
  pub balance: i128,                // s_userBalance - balance of user
  pub rewards_per_token_paid: i128, // s_userRewardPerTokenPaid - rewards per token that have been paid
  pub rewards_to_claim: i128,       // s_rewards - rewards that can be claimed
}

pub struct StorageClient;

impl StorageClient {
  pub fn get_default_global_state(env: Env) -> StakingGlobals {
    StakingGlobals {
      token_supply: 0,
      reward_per_token_stored: 0,
      last_updated_time: 0,
    }
  }

  pub fn get_default_state(env: Env) -> StakingContractState {
    StakingContractState {
      reward_token: env.current_contract_address().clone(),
      base_token: env.current_contract_address().clone(),
      share_token: env.current_contract_address().clone(),
      owner: env.current_contract_address().clone(),
      initialized: false,
    }
  }

  pub fn get_default_user(env: Env, user: Address) -> UserRecord {
    UserRecord {
      balance: 0,
      rewards_per_token_paid: 0,
      rewards_to_claim: 0,
      address: user.clone(),
    }
  }
}
