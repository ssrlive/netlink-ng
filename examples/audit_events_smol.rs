// SPDX-License-Identifier: MIT

// Build:
//
// ```
// cd netlink-sys
// cargo build --example audit_events_smol --features smol_socket
// ```
//
// Run *as root*:
//
// ```
// ../target/debug/examples/audit_events_smol
// ```
//
// This example uses `SmolSocket` (backed by `async-io`) with the async-std
// runtime. Any runtime that drives the `async-io` reactor can be used.

use std::process;

use netlink_packet_audit::{AuditMessage, AuditStatus, StatusMessage};
use netlink_packet_core::{
    Emitable, NLM_F_ACK, NLM_F_REQUEST, NetlinkBuffer, NetlinkMessage,
};

use netlink_sys::{AsyncSocket, AsyncSocketExt, SmolSocket, SocketAddr};

const NETLINK_AUDIT: isize = 9;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kernel_unicast: SocketAddr = SocketAddr::new(0, 0);
    let socket = SmolSocket::new(NETLINK_AUDIT)?;

    let mut status = StatusMessage::new();
    status.enabled = 1;
    status.pid = process::id();
    status.mask = AuditStatus::Enabled | AuditStatus::Pid;
    let payload = AuditMessage::SetStatus(status);
    let mut nl_msg = NetlinkMessage::from(payload);
    nl_msg.header.flags = NLM_F_REQUEST | NLM_F_ACK;
    let mut buf = Vec::new();
    nl_msg.emit(&mut buf)?;

    println!(">>> {:?}", nl_msg);
    socket.send_to(&buf, &kernel_unicast).await?;

    let mut buf = vec![0u8; 1024 * 8];
    loop {
        let (n, _addr) = socket.recv_from(&mut buf).await?;
        // This dance with the NetlinkBuffer should not be
        // necessary. It is here to work around a netlink bug. See:
        // https://github.com/mozilla/libaudit-go/issues/24
        // https://github.com/linux-audit/audit-userspace/issues/78
        {
            let mut nl_buf = NetlinkBuffer::new(&mut buf[..n])?;
            if n != nl_buf.length() as usize {
                nl_buf.set_length(n as u32);
            }
        }
        let parsed = NetlinkMessage::<AuditMessage>::deserialize(&buf[..n])?;
        println!("<<< {:?}", parsed);
    }
}
