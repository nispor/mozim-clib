// SPDX-License-Identifier: Apache-2.0

use std::ffi::{c_char, CStr};

use mozim::DhcpV4Config;

use crate::{MOZIM_FAIL_INVALID_STR, MOZIM_FAIL_NULL_POINTER, MOZIM_PASS};

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn mozim_dhcpv4_config_new(
    config: *mut *mut DhcpV4Config,
    iface_name: *const c_char,
) -> u32 {
    if config.is_null() {
        return MOZIM_FAIL_NULL_POINTER;
    }

    unsafe {
        *config = std::ptr::null_mut();
    }

    let iface_name = unsafe { CStr::from_ptr(iface_name) };

    match iface_name.to_str() {
        Ok(iface_name) => unsafe {
            *config = Box::into_raw(Box::new(DhcpV4Config::new(iface_name)));
            MOZIM_PASS
        },
        Err(_) => MOZIM_FAIL_INVALID_STR,
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn mozim_dhcpv4_config_free(config: *mut DhcpV4Config) {
    if !config.is_null() {
        unsafe {
            drop(Box::from_raw(config));
        }
    }
}
