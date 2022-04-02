// Copyright 2022 Alessandro Cresto Miseroglio.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use socket2::{Domain, SockAddr, Socket};

pub fn new_socket(addr: &SocketAddr) -> io::Result<Socket> {
    let domain = if addr.is_ipv4() {
        Domain::IPV4
    } else {
        Domain::IPV6
    };

    let sock = Socket::new(domain, crate::TYPE, Some(crate::PROTOCOL))?;

    sock.set_read_timeout(Some(Duration::from_millis(100)))?;

    Ok(sock)
}

pub fn join_multicast(addr: SocketAddr) -> io::Result<Socket> {
    let ip_addr = addr.ip();

    let sock = new_socket(&addr)?;

    match ip_addr {
        IpAddr::V4(ref mdns_v4) => {
            sock.join_multicast_v4(mdns_v4, &Ipv4Addr::new(0, 0, 0, 0))?;
        }
        IpAddr::V6(ref mdns_v6) => {
            sock.join_multicast_v6(mdns_v6, 0)?;
            sock.set_only_v6(true)?;
        }
    };

    sock.bind(&SockAddr::from(addr))?;
    Ok(sock)
}
