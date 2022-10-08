use crate::ft::{FungibleToken, FtMint, FtBurn};
use near_sdk::collections::LookupMap;
use near_sdk::{
  env, ext_contract, log, require, AccountId, Balance, Gas, IntoStorageKey,
  PromiseOrValue, PromiseResult, StorageUsage, BorshStorageKey
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::ft::vote::events::RepVote;
use near_sdk::json_types::U128;

const VOTE_CUP: u128 = 2;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
  VotesPerAccount { account_hash: Vec<u8> },
}

pub(crate) fn current_day() -> u64 {
  let now = env::block_timestamp() as u64;
  let day = 86400000 * 1000000;

  now / day
}

impl FungibleToken {
  pub(crate) fn internal_vote_used_of(&self, account_id: &AccountId, day: &u64) -> u128 {
    let votes = self.vote_by_account.get(&account_id);

    if let Some(votes) = votes {
      return votes.get(day).unwrap_or_else(|| 0);
    }

    0
  }

  pub(crate) fn internal_vote_balance(&self, account_id: &AccountId) -> (u64, u128) {
    let day = current_day();

    let used = self.internal_vote_used_of(&account_id, &day);

    (day, VOTE_CUP - used)
  }

  pub(crate) fn internal_decrease_vote(&mut self, sender_id: &AccountId, amount: &u128) -> u128 {
    let day = current_day();

    let mut votes = self.vote_by_account.get(&sender_id).unwrap_or_else(||
      LookupMap::new(StorageKey::VotesPerAccount {
        account_hash: env::sha256(sender_id.as_bytes()),
      })
    );

    let used = votes.get(&day).unwrap_or_else(|| 0);
    let balance = VOTE_CUP - used;

    if balance < amount.clone() {
      env::panic_str("All daily votes used");
    }

    let next_used = used + amount;
    votes.insert(&day, &next_used);
    self.vote_by_account.insert(&sender_id, &votes);

    balance - amount
  }

  pub(crate) fn internal_vote(&mut self, sender_id: &AccountId, receiver_id: &AccountId, amount: &u128, up: bool) -> u128 {
    assert_ne!(sender_id, receiver_id, "Only for another users");

    let next_votes = self.internal_decrease_vote(&sender_id, &amount);

    if up {
      self.internal_deposit(&receiver_id, amount.clone());

      RepVote {
        sender_id,
        receiver_id,
        amount: &U128::from(amount.clone()),
        memo: None,
        up: &up,
      }.emit();
      FtMint {
        owner_id: receiver_id,
        amount: &U128::from(amount.clone()),
        memo: Some("mint by vote"),
      }.emit();
    } else {
      let balance = self.internal_unwrap_balance_of(&receiver_id);
      let value = if balance >= amount.clone() { amount.clone() } else { balance };

      self.internal_withdraw(&receiver_id, value.clone());

      RepVote {
        sender_id,
        receiver_id,
        amount: &U128::from(value.clone()),
        memo: None,
        up: &up
      }.emit();
      FtBurn {
        owner_id: receiver_id,
        amount: &U128::from(value.clone()),
        memo: Some("burn by vote"),
      }.emit();
    }

    let voted = self.voted_by_account.get(&sender_id).unwrap_or_else(|| 0);
    self.voted_by_account.insert(&sender_id, &(voted + 1));

    next_votes
  }
}
