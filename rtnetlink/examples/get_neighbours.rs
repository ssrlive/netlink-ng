// SPDX-License-Identifier: MIT

#[cfg(target_os = "linux")]
use futures_util::stream::TryStreamExt;
#[cfg(target_os = "linux")]
use netlink_packet_route::{neighbour::NeighbourFlags, AddressFamily};
#[cfg(target_os = "linux")]
use rtnetlink::{new_connection, Error, Handle};

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() -> Result<(), ()> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    println!("dumping neighbours");
    if let Err(e) = dump_neighbours(handle.clone()).await {
        eprintln!("{e}");
    }
    println!();
    println!("dumping neighbours on bridge interfaces");
    if let Err(e) = dump_neighbours_bridge(handle.clone()).await {
        eprintln!("{e}");
    }
    println!();

    Ok(())
}

#[cfg(target_os = "linux")]
async fn dump_neighbours(handle: Handle) -> Result<(), Error> {
    let mut neighbours = handle
        .neighbours()
        .get()
        .set_address_family(AddressFamily::Inet)
        .execute();
    while let Some(route) = neighbours.try_next().await? {
        println!("{route:?}");
    }
    Ok(())
}

#[cfg(target_os = "linux")]
async fn dump_neighbours_bridge(handle: Handle) -> Result<(), Error> {
    let mut neighbours = handle
        .neighbours()
        .get()
        .set_address_family(AddressFamily::Bridge)
        .set_flags(NeighbourFlags::Own)
        .execute();
    while let Some(route) = neighbours.try_next().await? {
        println!("{route:?}");
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() -> Result<(), &'static str> {
    Err("this example requires Linux")
}
