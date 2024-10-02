use std::{fmt, net::IpAddr};

#[derive(Debug)]
pub struct Connection {
    pub addr: IpAddr,
    pub port: u16,
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.addr, self.port)
    }
}
