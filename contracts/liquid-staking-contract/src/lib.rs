#![no_std]
use core::panic;
use errors::Error;
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, IntoVal, String};
use storage::UserInfoRegistry;

#[contract]
pub struct LiquidStakingContract;

#[contractimpl]
impl LiquidStakingContract {
  pub fn initialize(
    env: Env,
    base_token: Address,
    reward_token: Address,
    owner: Address,
    token_wasm_hash: BytesN<32>,
  ) -> Result<storage::StakingContractState, Error> {
    owner.require_auth();

    let mut state = env
      .storage()
      .instance()
      .get(&storage::STAKING_STATE)
      .unwrap_or(storage::StorageClient::get_default_state(env.clone()));

    if state.initialized {
      return Err(Error::AlreadyInitialized);
    }

    let token_contract = token::create_contract(&env, token_wasm_hash, &base_token, &reward_token);
    let token_name: String = "Staked XLM".into_val(&env);
    let token_symbol: String = "stXLM".into_val(&env);

    let client = token::Client::new(&env, &token_contract);

    client.initialize(
      &env.current_contract_address(),
      &7u32,
      &token_name,
      &token_symbol,
    );

    state.base_token = base_token;
    state.reward_token = reward_token;
    state.owner = owner.clone();
    state.share_token = token_contract;
    state.initialized = true;

    env
      .storage()
      .instance()
      .set(&storage::STAKING_STATE, &state);

    return Ok(state);
  }

  pub fn set_owner(env: Env, new_owner: Address, current_owner: Address) -> Result<(), Error> {
    current_owner.require_auth();
    new_owner.require_auth();

    let mut state = Self::get_staking_state(env.clone())
      .unwrap_or(storage::StorageClient::get_default_state(env.clone()));

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

    Ok(())
  }

  pub fn add_reward_funds(env: Env, owner: Address, amount: i128) -> Result<(), Error> {
    owner.require_auth();

    let state = Self::get_staking_state(env.clone())
      .unwrap_or(storage::StorageClient::get_default_state(env.clone()));

    if state.initialized == false {
      return Err(Error::NotInitialized);
    }

    if state.owner != owner {
      return Err(Error::NotOwner);
    }

    let reward_token = token::Client::new(&env, &state.reward_token);

    reward_token.transfer(&owner, &env.current_contract_address(), &amount);

    return Ok(());
  }

  pub fn earned(env: Env, user: Address) -> i128 {
    let state = Self::get_staking_state(env.clone())
      .unwrap_or(storage::StorageClient::get_default_state(env.clone()));

    if state.initialized == false {
      return 0;
    }

    let key = UserInfoRegistry::UserRecord(user.clone());

    let user_info = env
      .storage()
      .instance()
      .get(&key)
      .unwrap_or(storage::StorageClient::get_default_user(env.clone(), user));

    let current_balance = user_info.balance;

    let amount_paid = user_info.rewards_per_token_paid;

    let current_reward_per_token = Self::reward_per_token(env.clone());

    let past_rewards = user_info.rewards_to_claim;

    let decimals = 1e7 as i128;

    let earned =
      ((current_balance * (current_reward_per_token - amount_paid)) / decimals) + past_rewards;

    return earned;
  }

  fn reward_per_token(env: Env) -> i128 {
    // let state = env.storage().instance().get(storage::)
    let e = env.clone();
    let state = env
      .storage()
      .instance()
      .get(&storage::STAKING_STATE)
      .unwrap_or(storage::StorageClient::get_default_state(e.clone()));

    if state.initialized == false {
      return 0;
    }

    let global_state = env
      .storage()
      .instance()
      .get(&storage::STAKING_GLOBALS)
      .unwrap_or(storage::StorageClient::get_default_global_state(e.clone()));

    if global_state.token_supply == 0 {
      return global_state.reward_per_token_stored;
    } else {
      let current_timestamp = e.ledger().timestamp() as i128;

      let decimals = 1e7 as i128;

      return global_state.reward_per_token_stored
        + (((current_timestamp - global_state.last_updated_time)
          * constants::REWARD_RATE
          * decimals)
          / global_state.token_supply);
    }
  }

  fn update_reward(env: Env, user: Address) {
    let mut global_state = env
      .storage()
      .instance()
      .get(&storage::STAKING_GLOBALS)
      .unwrap_or(storage::StorageClient::get_default_global_state(
        env.clone(),
      ));

    global_state.reward_per_token_stored = Self::reward_per_token(env.clone());
    global_state.last_updated_time = env.ledger().timestamp() as i128;

    let key = UserInfoRegistry::UserRecord(user.clone());
    let mut user_info =
      env
        .storage()
        .instance()
        .get(&key)
        .unwrap_or(storage::StorageClient::get_default_user(
          env.clone(),
          user.clone(),
        ));

    user_info.rewards_to_claim = Self::earned(env.clone(), user); // TODO: Add Earned function and call it here
    user_info.rewards_per_token_paid = global_state.reward_per_token_stored;

    env.storage().instance().set(&key, &user_info);

    env
      .storage()
      .instance()
      .set(&storage::STAKING_GLOBALS, &global_state);
  }

  pub fn unstake(env: Env, user: Address, amount: i128) -> Result<(), Error> {
    user.require_auth();
    if amount < 0 {
      return Err(Error::InvalidAmount);
    }

    let state = Self::get_staking_state(env.clone())
      .unwrap_or(storage::StorageClient::get_default_state(env.clone()));

    if state.initialized == false {
      return Err(Error::NotInitialized);
    }

    let share_token_client = token::Client::new(&env, &state.share_token);

    if share_token_client.balance(&user) < amount {
      return Err(Error::NotEnoughFunds);
    }

    let base_token_client = token::Client::new(&env, &state.base_token);

    let mut global_state = env
      .storage()
      .instance()
      .get(&storage::STAKING_GLOBALS)
      .unwrap_or(storage::StorageClient::get_default_global_state(
        env.clone(),
      ));

    let key = UserInfoRegistry::UserRecord(user.clone());

    let mut user_record = env.storage().instance().get(&key).unwrap_or({
      storage::UserRecord {
        balance: 0,
        rewards_per_token_paid: 0,
        rewards_to_claim: 0,
        address: user.clone(),
      }
    });

    if user_record.balance < amount {
      return Err(Error::NotEnoughFunds);
    }

    user_record.balance -= amount;
    global_state.token_supply -= amount;

    env.storage().instance().set(&key, &user_record);

    env
      .storage()
      .instance()
      .set(&storage::STAKING_GLOBALS, &global_state);

    Self::update_reward(env.clone(), user.clone());

    if user_record.balance == 0 {
      let _ = Self::claim_rewards(env.clone(), user.clone());

      env.storage().instance().remove(&key);

      return Ok(());
    }

    share_token_client.burn(&user, &amount);
    base_token_client.transfer(&env.current_contract_address(), &user, &amount);

    Ok(())
  }

  pub fn stake(env: Env, user: Address, amount: i128) -> Result<(), Error> {
    if amount < 0 {
      return Err(Error::InvalidAmount);
    }
    let state = Self::get_staking_state(env.clone())
      .unwrap_or(storage::StorageClient::get_default_state(env.clone()));

    if state.initialized == false {
      return Err(Error::NotInitialized);
    }

    let share_token_client = token::Client::new(&env, &state.share_token);

    let base_token_client = token::Client::new(&env, &state.base_token);

    if base_token_client.balance(&user) < amount {
      return Err(Error::NotEnoughFunds);
    }

    let mut global_state = env
      .storage()
      .instance()
      .get(&storage::STAKING_GLOBALS)
      .unwrap_or(storage::StorageClient::get_default_global_state(
        env.clone(),
      ));

    global_state.token_supply += amount;

    let key = UserInfoRegistry::UserRecord(user.clone());

    let mut user_record = env.storage().instance().get(&key).unwrap_or({
      storage::UserRecord {
        balance: 0,
        rewards_per_token_paid: 0,
        rewards_to_claim: 0,
        address: user.clone(),
      }
    });

    user_record.balance += amount;

    Self::update_reward(env.clone(), user.clone());
    base_token_client.transfer(&user, &env.current_contract_address(), &amount);
    share_token_client.mint(&user, &amount);

    return Ok(());
  }

  pub fn claim_rewards(env: Env, user: Address) -> Result<(), Error> {
    user.require_auth();

    let state = Self::get_staking_state(env.clone())
      .unwrap_or(storage::StorageClient::get_default_state(env.clone()));

    if state.initialized == false {
      return Err(Error::NotInitialized);
    }

    let reward_token = token::Client::new(&env, &state.reward_token);

    let key = UserInfoRegistry::UserRecord(user.clone());

    let mut user_record = env.storage().instance().get(&key).unwrap_or({
      storage::UserRecord {
        balance: 0,
        rewards_per_token_paid: 0,
        rewards_to_claim: 0,
        address: user.clone(),
      }
    });

    if user_record.rewards_to_claim == 0 {
      return Err(Error::ThereIsNoRewardToClaim);
    }

    reward_token.transfer(
      &env.current_contract_address(),
      &user,
      &user_record.rewards_to_claim,
    );

    user_record.rewards_to_claim = 0;

    Ok(())
  }

  pub fn get_user_position(env: Env, user: Address) -> storage::UserRecord {
    user.require_auth();

    let key = UserInfoRegistry::UserRecord(user.clone());

    let user_record = env
      .storage()
      .instance()
      .get(&key)
      .unwrap_or(storage::StorageClient::get_default_user(env.clone(), user));

    return user_record;
  }

  pub fn get_staking_state(env: Env) -> Result<storage::StakingContractState, Error> {
    let state = env
      .storage()
      .instance()
      .get(&storage::STAKING_STATE)
      .unwrap_or(storage::StorageClient::get_default_state(env.clone()));

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
