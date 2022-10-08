use near_sdk::{AccountId, Balance, env, log, near_bindgen, PanicOnDefault, PromiseOrValue, StorageUsage};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;

use ft::{
  FT_METADATA_SPEC, FungibleToken, FungibleTokenMetadata, FungibleTokenMetadataProvider
};

use crate::ft::{FtMint};

mod ft;
mod event;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  token: FungibleToken,
  metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg width='24' height='24' viewBox='0 0 24 24' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M6.17512 18.0234L5.93884 17.7027V16.2629L6.31818 15.9379L5.93884 16.0618V11.0351L5.10793 11.8686C5.02618 11.9505 5.02618 12.0835 5.10793 12.1655L5.67767 12.7369V14.2216L4.96902 13.5108L5.09091 13.0884L4.62552 13.1662L4.36786 12.9078C4.20596 12.7454 4.09751 12.5497 4.04249 12.3427L4.04545 12.3362L4.03902 12.3294C3.93313 11.9127 4.04275 11.4523 4.36786 11.1263L5.04795 10.4441H5.34091V10.1503L8.49431 6.98735H6.98546V8.13528L5.93884 9.18506V7.59159L6.31818 7.38946L5.93884 7.12088V5.93757H7.78204L8.06818 6.20407L8.56638 5.93757H9.54093L9.54343 5.93506L11.0236 5.93506L11.0211 5.93757H12.9789L12.148 5.10415C12.0663 5.02216 11.9337 5.02215 11.852 5.10415L11.2853 5.67252H9.80518L11.1119 4.36184C11.3231 4.15005 11.5907 4.02943 11.8662 4L12.8184 4.29694C12.8422 4.31753 12.8655 4.33916 12.8881 4.36184L13.1591 4.63366V4.72234L13.5455 5.42901L14.0334 5.51058L16.9416 8.42764V6.98735H15.8751L14.8285 5.93757H17.9883V6.34526L17.8636 6.47762L17.9883 6.76931V13.072L18.8921 12.1655C18.9738 12.0835 18.9738 11.9505 18.8921 11.8686L18.2495 11.224V9.73941L18.5904 10.0813L18.6364 10.1933L18.7823 10.2739L19.6321 11.1263C20.1226 11.6182 20.1226 12.4158 19.6321 12.9078L19.5896 12.9504L19.0909 13.0884V13.4507L15.5786 16.9736H16.9416V15.9694L17.9883 14.9196V18.0234H15.2599L15.1136 17.8983L14.7605 18.0234H10.9482L11.852 18.9299C11.9337 19.0119 12.0663 19.0119 12.148 18.9299L12.7915 18.2844H14.2045V18.3518L13.3157 19.2433H13.1591L12.8664 19.6934C12.835 19.7234 12.8025 19.7515 12.7689 19.7777L11.6808 20C11.4724 19.9453 11.2752 19.836 11.1119 19.6722L10.9939 19.5538L10.4545 18.8558V19.0128L6.98546 15.5333V16.9736H8.05476L9.10138 18.0234H7.23847L6.90909 17.8983L6.67764 18.0234H6.17512ZM16.9416 9.91225L14.0255 6.98735H9.97445L6.98546 9.98536V14.0487L9.90156 16.9736H14.0984L16.9416 14.1218V9.91225Z' fill='%23F98966'/%3E%3Cpath d='M11.5098 12.4451L11.8298 13.0931L11.9018 13.2771L12.1978 13.8371L12.2938 13.8851L12.2858 14.0211L12.5658 14.6611L12.7578 14.9731L13.4058 14.6131L13.4138 14.3811L13.1498 14.1011L13.0058 13.7971V13.8051L12.8938 13.5731L12.8458 13.5491V13.4451L12.5258 12.9011C12.4698 12.8211 12.4218 12.7811 12.3578 12.7091V12.5731C12.3098 12.4771 12.2138 12.3411 12.1578 12.2291L12.3818 11.9251L12.5178 11.7571L12.7738 11.4531L13.1418 11.0051L13.1978 10.5251L12.8138 10.1731C12.8058 10.1731 12.6778 10.2131 12.6698 10.2051L12.6218 10.0051L11.3498 8.90109L10.7178 9.15709L10.7498 9.84509L10.7898 11.3731L10.7258 11.5331L10.7418 11.9491L10.7738 12.8851V13.1971L10.8458 13.3011L10.7258 13.6131L10.6778 14.8851L11.2058 14.9331L11.5258 14.8051L11.5178 14.1411L11.4938 14.0691L11.4698 13.2691L11.4618 12.3971L11.5098 12.4451ZM11.4778 11.6931L11.4458 10.9811L11.4298 10.8931L11.4138 10.8451L11.3818 10.7011L11.4458 10.5491V10.4691L11.4618 9.85309L12.1978 10.5811L12.2538 10.6211L12.4138 10.8211L12.3258 10.9331C12.2218 11.0771 12.0298 11.3251 11.9178 11.4611L11.5178 11.9571L11.4618 11.9891L11.4778 11.6931Z' fill='%23F98966'/%3E%3C/svg%3E%0A";

#[near_bindgen]
impl Contract {
  /// Initializes the contract with the given total supply owned by the given `owner_id` with
  /// default metadata (for example purposes only).
  #[init]
  pub fn new_default_meta(owner_id: AccountId) -> Self {
    Self::new(
      owner_id,
      FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "Multiverse Fighters Reputation".to_string(),
        symbol: "MFREP".to_string(),
        icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
        reference: None,
        reference_hash: None,
        decimals: 0,
      },
    )
  }

  /// Initializes the contract with the given total supply owned by the given `owner_id` with
  /// the given fungible token metadata.
  #[init]
  pub fn new(
    owner_id: AccountId,
    metadata: FungibleTokenMetadata,
  ) -> Self {
    assert!(!env::state_exists(), "Already initialized");
    metadata.assert_valid();
    let mut this = Self {
      token: FungibleToken::new(b"a".to_vec(), b"v".to_vec(), b"v2".to_vec()),
      metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
    };
    this.token.internal_register_account(&owner_id);

    this
  }

  fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
    log!("Closed @{} with {}", account_id, balance);
  }

  fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
    log!("Account @{} burned {}", account_id, amount);
  }

  #[init(ignore_state)]
  #[private]
  pub fn migrate() -> Self {
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct OldFungibleToken {
      /// AccountID -> Account balance.
      pub accounts: LookupMap<AccountId, Balance>,

      /// Total supply of the all token.
      pub total_supply: Balance,

      /// The storage size in bytes for one account.
      pub account_storage_usage: StorageUsage,

      // AccountID -> Date > Vote count
      pub vote_by_account: LookupMap<AccountId, LookupMap<u64, u128>>,
      pub voted_by_account: LookupMap<AccountId, u128>,
    }
    #[derive(BorshDeserialize)]
    struct Old {
      token: OldFungibleToken,
      metadata: LazyOption<FungibleTokenMetadata>,
    }

    let old: Old = env::state_read().expect("Error");

    let token = FungibleToken {
      accounts: old.token.accounts,
      total_supply: old.token.total_supply,
      account_storage_usage: old.token.account_storage_usage,
      vote_by_account: old.token.vote_by_account,
      voted_by_account: old.token.voted_by_account,
    };

    Self {
      token,
      metadata: LazyOption::new(b"m".to_vec(), Some(& FungibleTokenMetadata{
        spec: FT_METADATA_SPEC.to_string(),
        name: "Multiverse Fighters Reputation".to_string(),
        symbol: "MFREP".to_string(),
        icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
        reference: None,
        reference_hash: None,
        decimals: 0,
      })),
    }
  }
}

// macros

impl_fungible_token_core!(Contract, token, on_tokens_burned);
impl_fungible_token_storage!(Contract, token, on_account_closed);
impl_reputation_vote!(Contract, token);

// impl

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
  fn ft_metadata(&self) -> FungibleTokenMetadata {
    self.metadata.get().unwrap()
  }
}
