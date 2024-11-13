#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{log, Address, BytesN, Env};

use crate::token;

soroban_sdk::contractimport!(file = "token/soroban_token_contract.wasm");

fn create_token_contract<'a>(e: &'a Env, admin: &'a Address) -> token::Client<'a> {
  let asset_contract_registration = e.register_stellar_asset_contract_v2(admin.clone());

  return token::Client::new(e, &asset_contract_registration.address());
}

fn install_contract_wasm(e: &Env) -> BytesN<32> {
  e.deployer().upload_contract_wasm(WASM)
}

fn create_liquid_staking_contract<'a>(e: &Env) -> contract::LiquidStakingContractClient<'a> {
  let staking_pool = contract::LiquidStakingContractClient::new(
    e,
    &e.register_contract(None, contract::LiquidStakingContract {}),
  );

  return staking_pool;
}

#[test]
fn test_contract_initialize() {
  let env = Env::default();

  env.mock_all_auths();

  let owner = Address::generate(&env);
  let base_token = create_token_contract(&env, &owner);
  let reward_token = create_token_contract(&env, &owner);

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  let initialized_state = liquid_staking_contract_client.initialize(
    &base_token.address.clone(),
    &reward_token.address.clone(),
    &owner,
    &install_contract_wasm(&env),
  );

  assert!(initialized_state.initialized);
  assert_eq!(initialized_state.owner, owner);
  assert_eq!(initialized_state.base_token, base_token.address);
  assert_ne!(initialized_state.share_token, base_token.address);
}

#[test]
fn test_staker_add_funds() {
  let env = Env::default();

  env.mock_all_auths();

  let owner = Address::generate(&env);

  let staker = Address::generate(&env);

  let base_token = create_token_contract(&env, &owner);
  let reward_token = create_token_contract(&env, &owner);

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  liquid_staking_contract_client.initialize(
    &base_token.address.clone(),
    &reward_token.address.clone(),
    &owner,
    &install_contract_wasm(&env),
  );

  reward_token.mint(&owner, &1000);

  liquid_staking_contract_client.add_reward_funds(&owner, &1000);

  let global_state = liquid_staking_contract_client.get_global_state();

  base_token.mint(&staker, &1000);

  let amount = 1000 as i128;

  liquid_staking_contract_client.stake(&staker, &amount);

  let user_position = liquid_staking_contract_client.get_user_position(&staker);

  log!(&env, "USER BALANCE", user_position.balance);
  log!(&env, "GLOBAL TOTAL SUPPLY", global_state.token_supply);

  assert!(user_position.balance > 0);
  assert_eq!(user_position.balance, amount);
}

#[test]
fn test_unstake_funds() {
  let env = Env::default();

  env.mock_all_auths();

  let owner = Address::generate(&env);

  let staker = Address::generate(&env);

  let base_token = create_token_contract(&env, &owner);
  let reward_token = create_token_contract(&env, &owner);

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  liquid_staking_contract_client.initialize(
    &base_token.address.clone(),
    &reward_token.address.clone(),
    &owner,
    &install_contract_wasm(&env),
  );

  reward_token.mint(&owner, &1000);

  liquid_staking_contract_client.add_reward_funds(&owner, &1000);

  base_token.mint(&staker, &1000);

  let amount = 1000 as i128;

  liquid_staking_contract_client.stake(&staker, &amount);

  let user_pos1 = liquid_staking_contract_client.get_user_position(&staker);

  liquid_staking_contract_client.unstake(&staker, &amount);

  let user_pos2 = liquid_staking_contract_client.get_user_position(&staker);

  assert!(user_pos1.balance > 0);
  assert_eq!(user_pos1.balance, amount);
  assert!(user_pos2.balance == 0);
}

#[test]
fn test_contract_owner_add_funds() {
  let env = Env::default();

  env.mock_all_auths();

  let owner = Address::generate(&env);

  let base_token = create_token_contract(&env, &owner);
  let reward_token = create_token_contract(&env, &owner);

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  liquid_staking_contract_client.initialize(
    &base_token.address.clone(),
    &reward_token.address.clone(),
    &owner,
    &install_contract_wasm(&env),
  );

  reward_token.mint(&owner, &1000);

  liquid_staking_contract_client.add_reward_funds(&owner, &1000);

  let state = liquid_staking_contract_client.get_staking_state();

  let balance = reward_token.balance(&liquid_staking_contract_client.address);

  assert!(state.initialized);
  assert_eq!(state.owner, owner);
  assert_eq!(state.base_token, base_token.address);
  assert_eq!(state.reward_token, reward_token.address);
  assert_ne!(state.share_token, base_token.address);

  assert_eq!(balance, 1000);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_contract_owner_add_funds_not_owner() {
  let env = Env::default();

  env.mock_all_auths();
  env.budget().reset_unlimited();

  let owner = Address::generate(&env);

  let base_token = create_token_contract(&env, &owner);
  let reward_token = create_token_contract(&env, &owner);

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  liquid_staking_contract_client.initialize(
    &base_token.address.clone(),
    &reward_token.address.clone(),
    &owner,
    &install_contract_wasm(&env),
  );

  reward_token.mint(&owner, &1000);

  let fake_owner = Address::generate(&env);

  liquid_staking_contract_client.add_reward_funds(&fake_owner, &1000);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_contract_owner_add_funds_contract_not_initialized() {
  let env = Env::default();

  env.mock_all_auths();
  env.budget().reset_unlimited();

  let owner = Address::generate(&env);

  let reward_token = create_token_contract(&env, &owner);

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  reward_token.mint(&owner, &1000);

  liquid_staking_contract_client.add_reward_funds(&owner, &1000);
}

#[test]
fn test_get_staking_state() {
  let env = Env::default();
  env.mock_all_auths();

  let owner = Address::generate(&env);
  let base_token = create_token_contract(&env, &owner);
  let reward_token = create_token_contract(&env, &owner);

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  liquid_staking_contract_client.initialize(
    &base_token.address.clone(),
    &reward_token.address.clone(),
    &owner,
    &install_contract_wasm(&env),
  );

  let state = liquid_staking_contract_client.get_staking_state();

  assert!(state.initialized);
  assert_eq!(state.owner, owner);
  assert_eq!(state.base_token, base_token.address);
  assert_eq!(state.reward_token, reward_token.address);
  assert_ne!(state.share_token, base_token.address);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_get_staking_state_not_initialized() {
  let env = Env::default();
  env.mock_all_auths();

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  liquid_staking_contract_client.get_staking_state();
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_contract_initialize_twice() {
  let env = Env::default();
  env.mock_all_auths();

  let owner = Address::generate(&env);

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  liquid_staking_contract_client.initialize(&owner, &owner, &owner, &install_contract_wasm(&env));
  liquid_staking_contract_client.initialize(&owner, &owner, &owner, &install_contract_wasm(&env));
}

#[test]
fn test_set_owner() {
  let env = Env::default();
  env.mock_all_auths();

  let owner = Address::generate(&env);

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  let base_token = create_token_contract(&env, &owner);
  let reward_token = create_token_contract(&env, &owner);

  liquid_staking_contract_client.initialize(
    &base_token.address.clone(),
    &reward_token.address.clone(),
    &owner,
    &install_contract_wasm(&env),
  );

  let new_owner = Address::generate(&env);

  liquid_staking_contract_client.set_owner(&new_owner, &owner);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_set_owner_not_owner() {
  let env = Env::default();

  let owner = Address::generate(&env);

  env.mock_all_auths();

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  let base_token = create_token_contract(&env, &owner);
  let reward_token = create_token_contract(&env, &owner);

  liquid_staking_contract_client.initialize(
    &base_token.address.clone(),
    &reward_token.address.clone(),
    &owner,
    &install_contract_wasm(&env),
  );

  let new_owner = Address::generate(&env);
  let non_owner = Address::generate(&env);

  liquid_staking_contract_client.set_owner(&new_owner, &non_owner);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_set_owner_not_initialized() {
  let env = Env::default();
  let owner = Address::generate(&env);

  let new_owner = Address::generate(&env);

  env.mock_all_auths();

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  liquid_staking_contract_client.set_owner(&new_owner, &owner);
}
