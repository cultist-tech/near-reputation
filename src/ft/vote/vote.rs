use near_sdk::json_types::U128;
use near_sdk::AccountId;
use near_sdk::PromiseOrValue;

pub trait ReputationTokenVote {
  fn rep_vote_of(&self, account_id: AccountId) -> u128;
  fn rep_voted_of(&self, account_id: AccountId) -> u128;

  fn rep_vote(&mut self, receiver_id: AccountId, amount: u128, up: bool) -> u128;
}
