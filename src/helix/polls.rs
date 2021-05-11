//! Helix endpoints regarding channel polls

use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};

pub mod create_poll;
pub mod end_poll;
pub mod get_polls;

#[doc(inline)]
pub use create_poll::{CreatePollBody, CreatePollRequest, NewPollChoice};
#[doc(inline)]
pub use end_poll::{EndPollBody, EndPollRequest};
#[doc(inline)]
pub use get_polls::{GetPollsRequest, Poll};
