// SPDX-License-Identifier: Apache-2.0

#[allow(clippy::not_unsafe_ptr_arg_deref)]
mod client;
#[allow(clippy::not_unsafe_ptr_arg_deref)]
mod config;
#[allow(clippy::not_unsafe_ptr_arg_deref)]
mod event;
#[allow(clippy::not_unsafe_ptr_arg_deref)]
mod lease;
mod logger;

pub use crate::client::{
    mozim_dhcpv4_client_free, mozim_dhcpv4_client_get_fd,
    mozim_dhcpv4_client_init, mozim_dhcpv4_client_poll,
    mozim_dhcpv4_client_process,
};
pub use crate::config::{mozim_dhcpv4_config_free, mozim_dhcpv4_config_new};
pub use crate::event::mozim_events_free;
pub use crate::lease::{
    mozim_dhcpv4_lease_free, mozim_dhcpv4_lease_get_gateway,
    mozim_dhcpv4_lease_get_gateway_count, mozim_dhcpv4_lease_get_lease_time,
    mozim_dhcpv4_lease_get_prefix_length, mozim_dhcpv4_lease_get_yiaddr,
};

pub(crate) const MOZIM_PASS: u32 = 0;
pub(crate) const MOZIM_FAIL: u32 = 1;
pub(crate) const MOZIM_FAIL_NULL_POINTER: u32 = 2;
pub(crate) const MOZIM_FAIL_INVALID_STR: u32 = 3;

use std::ffi::CString;
use std::os::raw::c_char;

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn mozim_cstring_free(cstring: *mut c_char) {
    unsafe {
        if !cstring.is_null() {
            drop(CString::from_raw(cstring));
        }
    }
}
