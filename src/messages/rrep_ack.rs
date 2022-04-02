// Copyright 2022 Alessandro Cresto Miseroglio.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::messages::headers::Type;

#[allow(dead_code)]
pub struct RouteResponseAck {
    ty: Type,
}

impl Default for RouteResponseAck {
    fn default() -> Self {
        Self { ty: Type::Ack }
    }
}
