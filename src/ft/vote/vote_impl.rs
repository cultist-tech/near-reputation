use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{
    env, ext_contract, log, require, AccountId, Balance, Gas, IntoStorageKey,
    PromiseOrValue, PromiseResult, StorageUsage, BorshStorageKey
};
use crate::ft::FungibleToken;
use crate::ft::vote::ReputationTokenVote;
use crate::ft::vote::events::RepVote;

impl ReputationTokenVote for FungibleToken {
  fn rep_vote_of(&self, account_id: AccountId) -> u128 {
    let (day, balance) = self.internal_vote_balance(&account_id);

    balance
  }

  fn rep_voted_of(&self, account_id: AccountId) -> u128 {
    self.voted_by_account.get(&account_id).unwrap_or_else(|| (0 as u128))
  }

  fn rep_vote(&mut self, receiver_id: AccountId, amount: u128, up: bool) -> u128 {
    self.internal_vote(&env::predecessor_account_id(), &receiver_id, &amount, up)
  }
}
