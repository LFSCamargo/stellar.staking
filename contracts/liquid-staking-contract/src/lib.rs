#![no_std]
use core::panic;
use errors::Error;
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, IntoVal, String};

#[contract]
pub struct LiquidStakingContract;

#[contractimpl]
impl LiquidStakingContract {
  pub fn set_owner(env: Env, new_owner: Address, current_owner: Address) -> Result<(), Error> {
    current_owner.require_auth();
    new_owner.require_auth();

    let mut state = Self::get_staking_state(env.clone()).unwrap_or(storage::StakingContractState {
      staking_token: env.current_contract_address().clone(),
      reward_token: env.current_contract_address(),
      owner: env.current_contract_address(),
      initialized: false,
    });

    if state.initialized == false {
      return Err(Error::NotInitialized);
    }

    if state.owner != current_owner {
      return Err(Error::NotOwner);
    }

    state.owner = new_owner;

    env
      .storage()
      .instance()
      .set(&storage::STAKING_STATE, &state);

    Ok(()) // Return Ok(()) to indicate success
  }

  pub fn initialize_staking(
    env: Env,
    staking_token: Address,
    owner: Address,
    token_wasm_hash: BytesN<32>,
  ) -> Result<storage::StakingContractState, Error> {
    owner.require_auth();

    let original_token = staking_token;

    let mut state = env
      .storage()
      .instance()
      .get(&storage::STAKING_STATE)
      .unwrap_or(storage::StakingContractState {
        staking_token: original_token.clone(),
        owner,
        initialized: false,
        reward_token: original_token.clone(),
      });

    if state.initialized {
      return Err(Error::AlreadyInitialized);
    }

    let token_contract = token::create_contract(&env, token_wasm_hash, &original_token);
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

    env
      .storage()
      .instance()
      .set(&storage::STAKING_STATE, &state);

    return Ok(state);
  }

  pub fn get_staking_state(env: Env) -> Result<storage::StakingContractState, Error> {
    let state = env
      .storage()
      .instance()
      .get(&storage::STAKING_STATE)
      .unwrap_or(storage::StakingContractState {
        staking_token: env.current_contract_address().clone(),
        reward_token: env.current_contract_address(),
        owner: env.current_contract_address(),
        initialized: false,
      });

    if state.initialized == false {
      return Err(Error::NotInitialized);
    }

    return Ok(state);
  }
}

mod constants;
mod errors;
mod storage;
mod test;
mod token;
