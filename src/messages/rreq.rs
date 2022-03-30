// SPDX-License-Identifier: MIT

use std::net::{IpAddr, Ipv4Addr};

use crate::messages::headers::RequestHeader;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct RouteRequest {
    pub header: RequestHeader,
    pub id: u32,
    pub destination: IpAddr,
    pub dest_seq: u32,
    pub originator: IpAddr,
    pub orig_seq: u32,
}

impl Default for RouteRequest {
    fn default() -> Self {
        RouteRequest {
            header: RequestHeader::default(),
            id: 0,
            destination: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            dest_seq: 0,
            originator: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            orig_seq: 0,
        }
    }
}

impl From<&[u8]> for RouteRequest {
    fn from(_v: &[u8]) -> Self {
        Self::default()
    }
}
