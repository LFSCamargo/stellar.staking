#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{token, Address, Env};

fn create_token_contract<'a>(e: &'a Env, admin: &'a Address) -> token::Client<'a> {
  let asset_contract_registration = e.register_stellar_asset_contract_v2(admin.clone());

  return token::Client::new(e, &asset_contract_registration.address());
}

fn install_contract_wasm(e: &Env) -> BytesN<32> {
  soroban_sdk::contractimport!(file = "token/soroban_token_contract.wasm");

  e.deployer().upload_contract_wasm(WASM)
}

fn create_liquid_staking_contract<'a>(e: &Env) -> LiquidStakingContractClient<'a> {
  let staking_pool = LiquidStakingContractClient::new(
    e,
    &e.register_contract(None, crate::LiquidStakingContract {}),
  );

  return staking_pool;
}

#[test]
fn test_contract_initialize() {
  let env = Env::default();

  env.mock_all_auths();

  let owner = Address::generate(&env);
  let token = create_token_contract(&env, &owner);
  let liquid_staking_contract_client = create_liquid_staking_contract(&env);
  let initialized_state = liquid_staking_contract_client.initialize(
    &token.address.clone(),
    &owner,
    &install_contract_wasm(&env),
  );

  assert!(initialized_state.initialized);
  assert_eq!(initialized_state.owner, owner);
  assert_eq!(initialized_state.staking_token, token.address);
  assert_ne!(initialized_state.reward_token, token.address);
}

#[test]
fn test_get_staking_state() {
  let env = Env::default();
  env.mock_all_auths();

  let owner = Address::generate(&env);
  let token = create_token_contract(&env, &owner);

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  liquid_staking_contract_client.initialize(
    &token.address.clone(),
    &owner,
    &install_contract_wasm(&env),
  );

  let state = liquid_staking_contract_client.get_staking_state();

  assert!(state.initialized);
  assert_eq!(state.owner, owner);
  assert_eq!(state.staking_token, token.address);
  assert_ne!(state.reward_token, token.address);
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

  liquid_staking_contract_client.initialize(&owner, &owner, &install_contract_wasm(&env));

  liquid_staking_contract_client.initialize(&owner, &owner, &install_contract_wasm(&env));
}

#[test]
fn test_set_owner() {
  let env = Env::default();
  env.mock_all_auths();

  let owner = Address::generate(&env);

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  let token = create_token_contract(&env, &owner);

  liquid_staking_contract_client.initialize(
    &token.address.clone(),
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

  let token = create_token_contract(&env, &owner);

  liquid_staking_contract_client.initialize(
    &token.address.clone(),
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
