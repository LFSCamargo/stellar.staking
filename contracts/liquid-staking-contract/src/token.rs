#![allow(unused)]
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env};

pub fn create_contract(e: &Env, token_wasm_hash: BytesN<32>, staked_token: &Address) -> Address {
  let mut salt = Bytes::new(e);
  salt.append(&staked_token.to_xdr(e));
  let salt = e.crypto().sha256(&salt);

  e.deployer()
    .with_current_contract(salt)
    .deploy(token_wasm_hash)
}
