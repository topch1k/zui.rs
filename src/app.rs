use core::fmt;
use std::net::IpAddr;
#[derive(Debug, Default)]
pub struct App {
    pub state: AppState,
    pub connection: Option<Connection>,
    pub connection_input: String,
}
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

impl App {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection_input: connection.to_string(),
            connection: Some(connection),
            ..Default::default()
        }
    }
    pub fn connection_str(&self) -> String {
        match self.connection {
            Some(ref conn) => conn.to_string(),
            None => "".to_owned(),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum AppState {
    #[default]
    EstablishingConnection,
    EditingConnection,
    Tab(TabState),
}
#[derive(Debug, PartialEq)]
pub enum TabState {}
