use clap::Parser;

use crate::app::Connection;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[arg(short, long, default_value = "127.0.0.1")]
    pub addr: String,
    #[arg(short, long, default_value = "2181")]
    pub port: u16,
}

pub(crate) fn parse_cli() -> CliArgs {
    CliArgs::parse()
}

impl CliArgs {
    pub(crate) fn connection(&self) -> Connection {
        Connection {
            addr: self.addr.parse().unwrap(), //TODO: handle error
            port: self.port,
        }
    }
}
