// SPDX-License-Identifier: MIT

#[cfg(target_os = "linux")]
use std::env;

#[cfg(target_os = "linux")]
use rtnetlink::NetworkNamespace;

#[cfg(not(target_os = "linux"))]
fn main() -> Result<(), &'static str> {
    Err("this example requires Linux")
}

#[cfg(target_os = "linux")]
#[async_std::main]
async fn main() -> Result<(), String> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        return Ok(());
    }
    let ns_name = &args[1];

    NetworkNamespace::add(ns_name.to_string())
        .await
        .map_err(|e| format!("{e}"))
}

#[cfg(target_os = "linux")]
fn usage() {
    eprintln!(
        "usage:
    cargo run --example add_netns -- <ns_name>

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cd netlink-ip ; cargo build --example add_netns

Then find the binary in the target directory:

    cd ../target/debug/example ; sudo ./add_netns <ns_name>"
    );
}
