// SPDX-License-Identifier: MIT

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
