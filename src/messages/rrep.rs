// Copyright 2022 Alessandro Cresto Miseroglio.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::net::{IpAddr, Ipv4Addr};

use crate::messages::headers::ResponseHeader;

#[allow(dead_code)]
pub struct RouteResponse {
    header: ResponseHeader,
    destination: IpAddr,
    sequence: u32,
    originator: IpAddr,
    lifetime: u32,
}

impl Default for RouteResponse {
    fn default() -> Self {
        RouteResponse {
            header: ResponseHeader::default(),
            destination: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            sequence: 0,
            originator: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            lifetime: 0,
        }
    }
}
