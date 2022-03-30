// SPDX-License-Identifier: MIT

#![allow(dead_code)]
use crate::messages::headers::ErrorHeader;
use std::net::IpAddr;

#[derive(Debug, Copy, Clone)]
pub struct Unreachable {
    destination: IpAddr,
    sequence: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct RouteError {
    header: ErrorHeader,
    unreachable: Unreachable,
    addictional_unreachable: Option<Unreachable>,
}
