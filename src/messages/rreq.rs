// SPDX-License-Identifier: MIT

use std::net::{IpAddr, Ipv4Addr};

use crate::messages::headers::ReqHeader;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct RReq {
    pub header: ReqHeader,
    pub id: u32,
    pub destination: IpAddr,
    pub dest_seq: u32,
    pub originator: IpAddr,
    pub orig_seq: u32,
}

impl Default for RReq {
    fn default() -> Self {
        RReq {
            header: ReqHeader::default(),
            id: 0,
            destination: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            dest_seq: 0,
            originator: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            orig_seq: 0,
        }
    }
}

impl From<&[u8]> for RReq {
    fn from(_v: &[u8]) -> Self {
        Self::default()
    }
}
