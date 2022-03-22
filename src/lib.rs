// SPDX-License-Identifier: MIT

mod messages;

pub use messages::*;
use socket2::{Protocol, Type};

pub const PORT: u32 = 654;
pub const TYPE: Type = Type::DGRAM;
pub const PROTOCOL: Protocol = Protocol::UDP;
