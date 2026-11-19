// SPDX-License-Identifier: MIT

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[non_exhaustive]
pub enum AddressFamily {
    #[default]
    Unspec,
    Local,
    Unix,
    Inet,
    Ax25,
    Ipx,
    Appletalk,
    Netrom,
    Bridge,
    Atmpvc,
    X25,
    Inet6,
    Rose,
    Decnet,
    Netbeui,
    Security,
    Key,
    Route,
    Netlink,
    Packet,
    Ash,
    Econet,
    Atmsvc,
    Rds,
    Sna,
    Irda,
    Pppox,
    Wanpipe,
    Llc,
    Ib,
    Mpls,
    Can,
    Tipc,
    Bluetooth,
    Iucv,
    Rxrpc,
    Isdn,
    Phonet,
    Ieee802154,
    Caif,
    Alg,
    Nfc,
    Vsock,
    Kcm,
    Qipcrtr,
    Smc,
    Xdp,
    Mctp,
    Other(u8),
}

impl From<u8> for AddressFamily {
    fn from(d: u8) -> Self {
        match d {
            0 => Self::Unspec,
            1 => Self::Unix,
            2 => Self::Inet,
            3 => Self::Ax25,
            4 => Self::Ipx,
            5 => Self::Appletalk,
            6 => Self::Netrom,
            7 => Self::Bridge,
            8 => Self::Atmpvc,
            9 => Self::X25,
            10 => Self::Inet6,
            11 => Self::Rose,
            12 => Self::Decnet,
            13 => Self::Netbeui,
            14 => Self::Security,
            15 => Self::Key,
            16 => Self::Netlink,
            17 => Self::Packet,
            18 => Self::Ash,
            19 => Self::Econet,
            20 => Self::Atmsvc,
            21 => Self::Rds,
            22 => Self::Sna,
            23 => Self::Irda,
            24 => Self::Pppox,
            25 => Self::Wanpipe,
            26 => Self::Llc,
            27 => Self::Ib,
            28 => Self::Mpls,
            29 => Self::Can,
            30 => Self::Tipc,
            31 => Self::Bluetooth,
            32 => Self::Iucv,
            33 => Self::Rxrpc,
            34 => Self::Isdn,
            35 => Self::Phonet,
            36 => Self::Ieee802154,
            37 => Self::Caif,
            38 => Self::Alg,
            39 => Self::Nfc,
            40 => Self::Vsock,
            41 => Self::Kcm,
            42 => Self::Qipcrtr,
            43 => Self::Smc,
            44 => Self::Xdp,
            45 => Self::Mctp,
            _ => Self::Other(d),
        }
    }
}

impl From<AddressFamily> for u8 {
    fn from(v: AddressFamily) -> u8 {
        match v {
            AddressFamily::Unspec => 0,
            AddressFamily::Local | AddressFamily::Unix => 1,
            AddressFamily::Inet => 2,
            AddressFamily::Ax25 => 3,
            AddressFamily::Ipx => 4,
            AddressFamily::Appletalk => 5,
            AddressFamily::Netrom => 6,
            AddressFamily::Bridge => 7,
            AddressFamily::Atmpvc => 8,
            AddressFamily::X25 => 9,
            AddressFamily::Inet6 => 10,
            AddressFamily::Rose => 11,
            AddressFamily::Decnet => 12,
            AddressFamily::Netbeui => 13,
            AddressFamily::Security => 14,
            AddressFamily::Key => 15,
            AddressFamily::Route | AddressFamily::Netlink => 16,
            AddressFamily::Packet => 17,
            AddressFamily::Ash => 18,
            AddressFamily::Econet => 19,
            AddressFamily::Atmsvc => 20,
            AddressFamily::Rds => 21,
            AddressFamily::Sna => 22,
            AddressFamily::Irda => 23,
            AddressFamily::Pppox => 24,
            AddressFamily::Wanpipe => 25,
            AddressFamily::Llc => 26,
            AddressFamily::Ib => 27,
            AddressFamily::Mpls => 28,
            AddressFamily::Can => 29,
            AddressFamily::Tipc => 30,
            AddressFamily::Bluetooth => 31,
            AddressFamily::Iucv => 32,
            AddressFamily::Rxrpc => 33,
            AddressFamily::Isdn => 34,
            AddressFamily::Phonet => 35,
            AddressFamily::Ieee802154 => 36,
            AddressFamily::Caif => 37,
            AddressFamily::Alg => 38,
            AddressFamily::Nfc => 39,
            AddressFamily::Vsock => 40,
            AddressFamily::Kcm => 41,
            AddressFamily::Qipcrtr => 42,
            AddressFamily::Smc => 43,
            AddressFamily::Xdp => 44,
            AddressFamily::Mctp => 45,
            AddressFamily::Other(d) => d,
        }
    }
}

impl std::fmt::Display for AddressFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unspec => f.write_str("unspec"),
            Self::Local => f.write_str("local"),
            Self::Unix => f.write_str("unix"),
            Self::Inet => f.write_str("inet"),
            Self::Ax25 => f.write_str("ax25"),
            Self::Ipx => f.write_str("ipx"),
            Self::Appletalk => f.write_str("appletalk"),
            Self::Netrom => f.write_str("netrom"),
            Self::Bridge => f.write_str("bridge"),
            Self::Atmpvc => f.write_str("atmpvc"),
            Self::X25 => f.write_str("x25"),
            Self::Inet6 => f.write_str("inet6"),
            Self::Rose => f.write_str("rose"),
            Self::Decnet => f.write_str("decnet"),
            Self::Netbeui => f.write_str("netbeui"),
            Self::Security => f.write_str("security"),
            Self::Key => f.write_str("key"),
            Self::Route => f.write_str("route"),
            Self::Netlink => f.write_str("netlink"),
            Self::Packet => f.write_str("packet"),
            Self::Ash => f.write_str("ash"),
            Self::Econet => f.write_str("econet"),
            Self::Atmsvc => f.write_str("atmsvc"),
            Self::Rds => f.write_str("rds"),
            Self::Sna => f.write_str("sna"),
            Self::Irda => f.write_str("irda"),
            Self::Pppox => f.write_str("pppox"),
            Self::Wanpipe => f.write_str("wanpipe"),
            Self::Llc => f.write_str("llc"),
            Self::Ib => f.write_str("ib"),
            Self::Mpls => f.write_str("mpls"),
            Self::Can => f.write_str("can"),
            Self::Tipc => f.write_str("tipc"),
            Self::Bluetooth => f.write_str("bluetooth"),
            Self::Iucv => f.write_str("iucv"),
            Self::Rxrpc => f.write_str("rxrpc"),
            Self::Isdn => f.write_str("isdn"),
            Self::Phonet => f.write_str("phonet"),
            Self::Ieee802154 => f.write_str("ieee802154"),
            Self::Caif => f.write_str("caif"),
            Self::Alg => f.write_str("alg"),
            Self::Nfc => f.write_str("nfc"),
            Self::Vsock => f.write_str("vsock"),
            Self::Kcm => f.write_str("kcm"),
            Self::Qipcrtr => f.write_str("qipcrtr"),
            Self::Smc => f.write_str("smc"),
            Self::Xdp => f.write_str("xdp"),
            Self::Mctp => f.write_str("mctp"),
            Self::Other(d) => write!(f, "{d}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AddressFamily;

    #[test]
    fn maps_linux_address_families_from_wire_values() {
        assert_eq!(AddressFamily::from(7), AddressFamily::Bridge);
        assert_eq!(AddressFamily::from(28), AddressFamily::Mpls);
        assert_eq!(AddressFamily::from(45), AddressFamily::Mctp);
        assert_eq!(u8::from(AddressFamily::Bridge), 7);
        assert_eq!(u8::from(AddressFamily::Mpls), 28);
        assert_eq!(u8::from(AddressFamily::Mctp), 45);
    }

    #[test]
    fn preserves_unknown_wire_values() {
        assert_eq!(AddressFamily::from(255), AddressFamily::Other(255));
        assert_eq!(u8::from(AddressFamily::Other(255)), 255);
    }
}
