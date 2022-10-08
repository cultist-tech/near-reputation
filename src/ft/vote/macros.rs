/// The core methods for a basic fungible token. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_reputation_vote {
    ($contract: ident, $token: ident) => {
        use $crate::ft::ReputationTokenVote;

        #[near_bindgen]
        impl ReputationTokenVote for $contract {
            fn rep_vote_of(&self, account_id: AccountId) -> u128 {
              self.$token.rep_vote_of(account_id)
            }

            fn rep_voted_of(&self, account_id: AccountId) -> u128 {
              self.$token.rep_voted_of(account_id)
            }

            fn rep_vote(&mut self, receiver_id: AccountId, amount: u128, up: bool) -> u128 {
             self.$token.rep_vote(receiver_id, amount, up)
            }
        }
    };
}
