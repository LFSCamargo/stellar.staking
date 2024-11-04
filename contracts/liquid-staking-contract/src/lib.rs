#![no_std]
use core::panic;
use soroban_sdk::{
  contract, contractimpl, contracttype, symbol_short, Address, BytesN, Env, IntoVal, String, Symbol,
};
use token::create_contract;

#[contracttype]
#[derive(Clone)]
pub struct StakingContractState {
  pub staking_token: Address,
  pub reward_token: Address,
  pub owner: Address,
  pub initialized: bool,
}

const STAKING_STATE: Symbol = symbol_short!("state");

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

// const STAKING_POOL_INFO: Symbol = symbol_short!("poolinfo");

#[contracttype]
pub enum UserInfoRegistry {
  UserRecord(Address),
}

#[contracttype]
#[derive(Clone)]
pub struct UserRecord {
  pub address: Address,
  pub staked_amount: u64,
  pub rewards_claimed: u64,
  pub user_reward_per_token_staked: u64,
  pub created_at: u64,
  pub updated_at: u64,
}

// const STAKING_DURATION: u64 = 2592000;
// const REWARD_PER_TOKEN: u64 = 100_000_000_000_000_000;

#[contract]
pub struct LiquidStakingContract;

#[contractimpl]
impl LiquidStakingContract {
  pub fn initialize_staking(
    env: Env,
    staking_token: Address,
    owner: Address,
    token_wasm_hash: BytesN<32>,
  ) -> StakingContractState {
    owner.require_auth();

    let original_token = staking_token;

    let mut state = env
      .storage()
      .instance()
      .get(&STAKING_STATE)
      .unwrap_or(StakingContractState {
        staking_token: original_token.clone(),
        owner,
        initialized: false,
        reward_token: original_token.clone(),
      });

    if state.initialized {
      panic!("Already initialized");
    }

    let token_contract = create_contract(&env, token_wasm_hash, &original_token);
    let token_name: String = "Staked XLM".into_val(&env);
    let token_symbol: String = "stXLM".into_val(&env);

    token::Client::new(&env, &token_contract).initialize(
      &env.current_contract_address(),
      &7u32,
      &token_name,
      &token_symbol,
    );

    state.reward_token = token_contract;
    state.initialized = true;

    env.storage().instance().set(&STAKING_STATE, &state);

    return state;
  }

  pub fn get_staking_state(env: Env) -> StakingContractState {
    let state = env
      .storage()
      .instance()
      .get(&STAKING_STATE)
      .unwrap_or(StakingContractState {
        staking_token: env.current_contract_address().clone(),
        reward_token: env.current_contract_address(),
        owner: env.current_contract_address(),
        initialized: false,
      });

    return state;
  }
}

mod test;
mod token;
