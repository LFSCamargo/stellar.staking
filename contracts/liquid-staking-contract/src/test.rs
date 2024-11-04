#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{log, token, Address, Env};

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

  log!(&env, "owner: {}", owner);

  let token = create_token_contract(&env, &owner);

  log!(&env, "token: {}", token.address);

  let liquid_staking_contract_client = create_liquid_staking_contract(&env);

  let initialized_state = liquid_staking_contract_client.initialize_staking(
    &token.address.clone(),
    &owner,
    &install_contract_wasm(&env),
  );

  // If the state is initialized it should be true and the initialize function should panic when called again
  assert!(initialized_state.initialized);
  // Initialized state has the owner
  assert_eq!(initialized_state.owner, owner);
  // The staking token is the token that we initialized above
  assert_eq!(initialized_state.staking_token, token.address);
  // The reward token the contract initializes and the contract is the owner
  assert_ne!(initialized_state.reward_token, token.address);

  let staking_state = liquid_staking_contract_client.get_staking_state();

  assert_eq!(staking_state.staking_token, token.address);
  assert_eq!(staking_state.reward_token, initialized_state.reward_token);
  assert_eq!(staking_state.owner, owner);
  assert!(staking_state.initialized)
}
