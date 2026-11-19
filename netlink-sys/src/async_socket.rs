// SPDX-License-Identifier: MIT

use std::{
    io,
    task::{Context, Poll},
};

use crate::{Socket, SocketAddr};

/// Trait to support different async backends
pub trait AsyncSocket: Sized + Unpin {
    /// Access underyling [`Socket`]
    fn socket_ref(&self) -> &Socket;

    /// Enable or disable `NETLINK_EXT_ACK` on the underlying socket.
    fn set_ext_ack(&self, value: bool) -> io::Result<()> {
        self.socket_ref().set_ext_ack(value)
    }

    /// Return whether `NETLINK_EXT_ACK` is enabled.
    fn get_ext_ack(&self) -> io::Result<bool> {
        self.socket_ref().get_ext_ack()
    }

    /// Enable or disable `NETLINK_CAP_ACK` on the underlying socket.
    fn set_cap_ack(&self, value: bool) -> io::Result<()> {
        self.socket_ref().set_cap_ack(value)
    }

    /// Return whether `NETLINK_CAP_ACK` is enabled.
    fn get_cap_ack(&self) -> io::Result<bool> {
        self.socket_ref().get_cap_ack()
    }

    /// Enable or disable `NETLINK_GET_STRICT_CHK` on the underlying socket.
    fn set_netlink_get_strict_chk(&self, value: bool) -> io::Result<()> {
        self.socket_ref().set_netlink_get_strict_chk(value)
    }

    /// Wrapper for [`Socket::new`]
    fn new(protocol: isize) -> io::Result<Self>;

    /// Polling wrapper for [`Socket::send`]
    fn poll_send(
        &self,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>>;

    /// Polling wrapper for [`Socket::send_to`]
    fn poll_send_to(
        &self,
        cx: &mut Context<'_>,
        buf: &[u8],
        addr: &SocketAddr,
    ) -> Poll<io::Result<usize>>;

    /// Polling wrapper for [`Socket::recv`]
    ///
    /// Passes 0 for flags and returns the number of bytes read.
    fn poll_recv(
        &self,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>>;

    /// Polling wrapper for [`Socket::recv_from`]
    ///
    /// Passes 0 for flags and returns the number of bytes read along with the
    /// sender address.
    fn poll_recv_from(
        &self,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<(usize, SocketAddr)>>;

    /// Polling wrapper for [`Socket::recv_from_full`]
    ///
    /// Passes 0 for flags and returns the received data along with the sender
    /// address.
    fn poll_recv_from_full(
        &self,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<(Vec<u8>, SocketAddr)>>;
}
