use near_sdk::AccountId;
use near_sdk::json_types::U128;
use crate::event::{NearEvent};
use serde::Serialize;

/// Data to log for an FT burn event. To log this event, call [`.emit()`](FtBurn::emit).
#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct RepVote<'a> {
  pub sender_id: &'a AccountId,
  pub receiver_id: &'a AccountId,
  pub amount: &'a U128,
  pub up: &'a bool,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub memo: Option<&'a str>,
}

impl RepVote<'_> {
  /// Logs the event to the host. This is required to ensure that the event is triggered
  /// and to consume the event.
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  /// Emits an FT burn event, through [`env::log_str`](near_sdk::env::log_str),
  /// where each [`FtBurn`] represents the data of each burn.
  pub fn emit_many<'a>(data: &'a [RepVote<'a>]) {
    new_mfight_rep_v1(MfightRepEventKind::RepVote(data)).emit()
  }
}

#[derive(Serialize, Debug)]
pub(crate) struct MfightRepEvent<'a> {
  version: &'static str,
  #[serde(flatten)]
  event_kind: MfightRepEventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum MfightRepEventKind<'a> {
  RepVote(&'a [RepVote<'a>]),
}

fn new_mfight_rep<'a>(version: &'static str, event_kind: MfightRepEventKind<'a>) -> NearEvent<'a> {
  NearEvent::MfRep(MfightRepEvent { version, event_kind })
}

fn new_mfight_rep_v1(event_kind: MfightRepEventKind) -> NearEvent {
  new_mfight_rep("1.0.0", event_kind)
}
