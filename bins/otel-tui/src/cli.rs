use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use clap::{crate_version, Parser};

const DEFAULT_LISTENER_ADDRESS: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4317);

#[derive(Debug, Parser)]
#[command(version = cli_version(), about)]
pub struct Cli {
    /// Optional listener address
    #[arg(default_value_t = DEFAULT_LISTENER_ADDRESS)]
    pub(crate) address: SocketAddr,
}

fn cli_version() -> &'static str {
    Box::leak(
        format!(
            "{}-{}",
            crate::built_info::GIT_VERSION.expect("failed to acquire tag of commit id of HEAD"),
            crate_version!()
        )
        .into(),
    )
}
