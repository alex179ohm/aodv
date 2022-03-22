// SPDX-License-Identifier: MIT

use std::net::{IpAddr, Ipv4Addr};

use crate::messages::headers::{RepHeader, Type};

#[allow(dead_code)]
pub struct RouteResponse {
    header: RepHeader,
    destination: IpAddr,
    sequence: u32,
    originator: IpAddr,
    lifetime: u32,
}

impl Default for RouteResponse {
    fn default() -> Self {
        RouteResponse {
            header: RepHeader::default(),
            destination: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            sequence: 0,
            originator: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            lifetime: 0,
        }
    }
}

#[allow(dead_code)]
pub struct RRepAck {
    ty: Type,
}

impl Default for RRepAck {
    fn default() -> Self {
        Self { ty: Type::Ack }
    }
}
