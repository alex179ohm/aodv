// Copyright 2022 Alessandro Cresto Miseroglio <alessandro.cresto.miseroglio@gmail.com>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
