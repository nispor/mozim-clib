// SPDX-License-Identifier: Apache-2.0

mod logger;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::time::SystemTime;

use mozim::{
    DhcpError, DhcpV4Client, DhcpV4Config, DhcpV4Event, DhcpV4Lease, ErrorKind,
};
use once_cell::sync::OnceCell;

use crate::logger::MemoryLogger;

const MOZIM_PASS: u32 = 0;
const MOZIM_FAIL: u32 = 1;
const MOZIM_FAIL_NULL_POINTER: u32 = 2;
const MOZIM_FAIL_INVALID_STR: u32 = 3;

static INSTANCE: OnceCell<MemoryLogger> = OnceCell::new();

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
pub extern "C" fn mozim_dhcpv4_client_init(
    client: *mut *mut DhcpV4Client,
    config: *const DhcpV4Config,
    log: *mut *mut c_char,
    err_kind: *mut *mut c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if client.is_null() || err_kind.is_null() || err_msg.is_null() {
        return MOZIM_FAIL_NULL_POINTER;
    }

    unsafe {
        *client = std::ptr::null_mut();
        *err_kind = std::ptr::null_mut();
        *err_msg = std::ptr::null_mut();
    }

    let config: &DhcpV4Config = unsafe { &*config };

    let logger = match init_logger() {
        Ok(l) => l,
        Err(e) => {
            unsafe {
                *err_msg =
                    CString::new(format!("Failed to setup logger: {}", e))
                        .unwrap()
                        .into_raw();
            }
            return MOZIM_FAIL;
        }
    };
    let now = SystemTime::now();

    let result = DhcpV4Client::init(config.clone(), None);

    unsafe {
        *log = CString::new(logger.drain(now)).unwrap().into_raw();
    }

    match result {
        Ok(c) => unsafe {
            *client = Box::into_raw(Box::new(c));
            MOZIM_PASS
        },
        Err(e) => unsafe {
            *err_msg = CString::new(e.msg()).unwrap().into_raw();
            *err_kind =
                CString::new(format!("{}", &e.kind())).unwrap().into_raw();
            MOZIM_FAIL
        },
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn mozim_dhcpv4_client_poll(
    client: *mut DhcpV4Client,
    wait_time: u32,
    events: *mut *mut u64,
    event_count: *mut u64,
    log: *mut *mut c_char,
    err_kind: *mut *mut c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if client.is_null()
        || events.is_null()
        || event_count.is_null()
        || err_kind.is_null()
        || err_msg.is_null()
    {
        return MOZIM_FAIL_NULL_POINTER;
    }

    unsafe {
        *event_count = 0;
        *events = std::ptr::null_mut();
        *err_kind = std::ptr::null_mut();
        *err_msg = std::ptr::null_mut();
    }

    let client: &mut DhcpV4Client = unsafe { &mut *client };

    let logger = match init_logger() {
        Ok(l) => l,
        Err(e) => {
            unsafe {
                *err_msg =
                    CString::new(format!("Failed to setup logger: {}", e))
                        .unwrap()
                        .into_raw();
            }
            return MOZIM_FAIL;
        }
    };
    let now = SystemTime::now();

    let result = client.poll(wait_time.into());
    unsafe {
        *log = CString::new(logger.drain(now)).unwrap().into_raw();
    }

    match result {
        Ok(result_events) => {
            if !result_events.is_empty() {
                let result_events: Vec<u64> = result_events
                    .as_slice()
                    .iter()
                    .map(|e| *e as u64)
                    .collect();
                let event_ids_len = result_events.len() as u64;
                // We trust C library user to use `mozim_events_free()`
                let mut event_ids_box = result_events.into_boxed_slice();
                unsafe {
                    *event_count = event_ids_len;
                    *events = event_ids_box.as_mut_ptr();
                }
                std::mem::forget(event_ids_box);
            }
            MOZIM_PASS
        }
        Err(e) => unsafe {
            *err_msg = CString::new(e.msg()).unwrap().into_raw();
            *err_kind =
                CString::new(format!("{}", &e.kind())).unwrap().into_raw();
            MOZIM_FAIL
        },
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn mozim_dhcpv4_client_process(
    client: *mut DhcpV4Client,
    event: u64,
    lease: *mut *mut DhcpV4Lease,
    log: *mut *mut c_char,
    err_kind: *mut *mut c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if client.is_null()
        || lease.is_null()
        || err_kind.is_null()
        || err_msg.is_null()
    {
        return MOZIM_FAIL_NULL_POINTER;
    }

    unsafe {
        *lease = std::ptr::null_mut();
        *err_kind = std::ptr::null_mut();
        *err_msg = std::ptr::null_mut();
    }

    let client: &mut DhcpV4Client = unsafe { &mut *client };

    let logger = match init_logger() {
        Ok(l) => l,
        Err(e) => {
            unsafe {
                *err_msg =
                    CString::new(format!("Failed to setup logger: {}", e))
                        .unwrap()
                        .into_raw();
            }
            return MOZIM_FAIL;
        }
    };
    let now = SystemTime::now();

    let event = match DhcpV4Event::try_from(event) {
        Ok(e) => e,
        Err(e) => {
            unsafe {
                *err_msg = CString::new(e.msg()).unwrap().into_raw();
                *err_kind =
                    CString::new(format!("{}", &e.kind())).unwrap().into_raw();
            }
            return MOZIM_FAIL;
        }
    };

    let result = client.process(event);
    unsafe {
        *log = CString::new(logger.drain(now)).unwrap().into_raw();
    }

    match result {
        Ok(Some(l)) => {
            unsafe {
                *lease = Box::into_raw(Box::new(l));
            }
            MOZIM_PASS
        }
        Ok(None) => MOZIM_PASS,
        Err(e) => unsafe {
            *err_msg = CString::new(e.msg()).unwrap().into_raw();
            *err_kind =
                CString::new(format!("{}", &e.kind())).unwrap().into_raw();
            MOZIM_FAIL
        },
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

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn mozim_dhcpv4_client_free(client: *mut DhcpV4Client) {
    if !client.is_null() {
        unsafe {
            drop(Box::from_raw(client));
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn mozim_dhcpv4_lease_free(lease: *mut DhcpV4Lease) {
    if !lease.is_null() {
        unsafe {
            drop(Box::from_raw(lease));
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn mozim_cstring_free(cstring: *mut c_char) {
    unsafe {
        if !cstring.is_null() {
            drop(CString::from_raw(cstring));
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn mozim_events_free(events: *mut u64, event_count: u64) {
    unsafe {
        if !events.is_null() {
            let events_slice =
                std::slice::from_raw_parts_mut(events, event_count as usize);
            drop(Box::from_raw(events_slice));
        }
    }
}

#[no_mangle]
pub extern "C" fn mozim_dhcpv4_lease_get_t1(lease: *const DhcpV4Lease) -> u32 {
    let lease: &DhcpV4Lease = unsafe { &*lease };
    lease.t1
}

#[no_mangle]
pub extern "C" fn mozim_dhcpv4_lease_get_t2(lease: *const DhcpV4Lease) -> u32 {
    let lease: &DhcpV4Lease = unsafe { &*lease };
    lease.t2
}

#[no_mangle]
pub extern "C" fn mozim_dhcpv4_lease_get_yiaddr(
    lease: *const DhcpV4Lease,
) -> u32 {
    let lease: &DhcpV4Lease = unsafe { &*lease };
    u32::from(lease.yiaddr)
}

fn init_logger() -> Result<&'static MemoryLogger, DhcpError> {
    match INSTANCE.get() {
        Some(l) => {
            l.add_consumer();
            Ok(l)
        }
        None => {
            if INSTANCE.set(MemoryLogger::new()).is_err() {
                return Err(DhcpError::new(
                    ErrorKind::Bug,
                    "Failed to set once_sync for logger".to_string(),
                ));
            }
            if let Some(l) = INSTANCE.get() {
                if let Err(e) = log::set_logger(l) {
                    Err(DhcpError::new(
                        ErrorKind::Bug,
                        format!("Failed to log::set_logger: {}", e),
                    ))
                } else {
                    l.add_consumer();
                    log::set_max_level(log::LevelFilter::Debug);
                    Ok(l)
                }
            } else {
                Err(DhcpError::new(
                    ErrorKind::Bug,
                    "Failed to get logger from once_sync".to_string(),
                ))
            }
        }
    }
}
