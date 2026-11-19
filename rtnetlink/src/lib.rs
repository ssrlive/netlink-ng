// SPDX-License-Identifier: MIT

//! This crate provides methods to manipulate networking resources (links,
//! addresses, arp tables, route tables) via the netlink protocol.

#![allow(clippy::module_inception)]

pub use netlink_packet_core as packet_core;
pub use netlink_packet_route as packet_route;
pub use netlink_proto as proto;
pub use netlink_proto::sys;

mod addr;
mod addrlabel;
mod connection;
pub mod constants;
mod errors;
mod handle;
mod link;
mod macros;
mod multicast;
mod neighbour;
#[cfg(any(target_os = "linux", target_os = "android"))]
mod ns;
mod route;
mod rule;
#[cfg(any(target_os = "linux", target_os = "android"))]
mod traffic_control;

#[cfg(feature = "tokio_socket")]
pub use crate::connection::{
    new_connection, new_connection_with_strict_check, new_multicast_connection,
};
#[cfg(any(
    target_os = "linux",
    target_os = "fuchsia",
    target_os = "android"
))]
pub use crate::link::LinkBridgeVlan;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub use crate::ns::{NetworkNamespace, NETNS_PATH, NONE_FS, SELF_NS_PATH};
#[cfg(any(target_os = "linux", target_os = "android"))]
pub use crate::traffic_control::{
    QDiscDelRequest, QDiscGetRequest, QDiscHandle, QDiscNewRequest,
    TrafficChainGetRequest, TrafficChainHandle, TrafficClassGetRequest,
    TrafficClassHandle, TrafficFilterGetRequest, TrafficFilterHandle,
    TrafficFilterNewRequest,
};
pub use crate::{
    addr::{
        AddressAddRequest, AddressDelRequest, AddressGetRequest, AddressHandle,
        AddressMessageBuilder,
    },
    addrlabel::{
        AddrLabelAddRequest, AddrLabelDelRequest, AddrLabelGetRequest,
        AddrLabelHandle,
    },
    connection::{
        from_socket, new_connection_with_socket,
        new_multicast_connection_with_socket,
    },
    errors::Error,
    handle::Handle,
    link::{
        AfstatsRequest, LinkAddRequest, LinkAmt, LinkAssignNsidHandle,
        LinkBareudp, LinkBatAdv, LinkBond, LinkBondPort, LinkBridge,
        LinkBridgePort, LinkCan, LinkDelPropRequest, LinkDelRequest, LinkDsa,
        LinkDummy, LinkGeneve, LinkGetNsidHandle, LinkGetRequest, LinkGre,
        LinkGre6, LinkGtp, LinkHandle, LinkHsr, LinkIfb, LinkIp6Tnl, LinkIpIp,
        LinkIpVlan, LinkIpVtap, LinkIpoib, LinkMacSec, LinkMacVlan,
        LinkMacVtap, LinkMessageBuilder, LinkNetdevsim, LinkNetkit, LinkNlmon,
        LinkPfcp, LinkRmNet, LinkSetRequest, LinkSit, LinkTeam, LinkTun,
        LinkUnspec, LinkVcan, LinkVeth, LinkVirtWifi, LinkVlan, LinkVrf,
        LinkVti, LinkVti6, LinkVxcan, LinkVxlan, LinkWireguard, LinkWwan,
        LinkXfrm, QosMapping,
    },
    multicast::MulticastGroup,
    neighbour::{
        NeighbourAddRequest, NeighbourDelRequest, NeighbourGetRequest,
        NeighbourHandle,
    },
    route::{
        IpVersion, RouteAddRequest, RouteDelRequest, RouteGetRequest,
        RouteHandle, RouteMessageBuilder, RouteNextHopBuilder,
    },
    rule::{RuleAddRequest, RuleDelRequest, RuleGetRequest, RuleHandle},
};
