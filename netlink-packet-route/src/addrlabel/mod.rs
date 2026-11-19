// SPDX-License-Identifier: MIT

mod attribute;
mod header;
mod message;
#[cfg(all(test, not(target_os = "freebsd")))]
mod tests;

pub use self::{
    attribute::AddrLabelAttribute, header::AddrLabelHeader,
    message::AddrLabelMessage,
};
