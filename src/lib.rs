// Copyright 2022 Alessandro Cresto Miseroglio.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod messages;
mod net;

pub use messages::*;
use socket2::{Protocol, Type};

pub const PORT: u32 = 654;
pub const TYPE: Type = Type::DGRAM;
pub const PROTOCOL: Protocol = Protocol::UDP;

pub const RREQ_SIZE: usize = 192;
pub const RREP_SIZE: usize = 160;
pub const RERR_SIZE: usize = 160;
pub const RREP_ACK_SIZE: usize = 16;

pub const ACTIVE_ROUTE_TIMEOUT: usize = 0;
