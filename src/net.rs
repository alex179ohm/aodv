use std::{io, net::SocketAddr, time::Duration};

use socket2::{Domain, Socket};

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
