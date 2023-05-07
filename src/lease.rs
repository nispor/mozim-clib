// SPDX-License-Identifier: Apache-2.0

use mozim::DhcpV4Lease;

#[no_mangle]
pub extern "C" fn mozim_dhcpv4_lease_get_lease_time(
    lease: *const DhcpV4Lease,
) -> u32 {
    let lease: &DhcpV4Lease = unsafe { &*lease };
    lease.lease_time
}

#[no_mangle]
pub extern "C" fn mozim_dhcpv4_lease_get_prefix_length(
    lease: *const DhcpV4Lease,
) -> u32 {
    let lease: &DhcpV4Lease = unsafe { &*lease };
    u32::from(lease.subnet_mask).count_ones()
}

#[no_mangle]
pub extern "C" fn mozim_dhcpv4_lease_get_gateway_count(
    lease: *const DhcpV4Lease,
) -> usize {
    let lease: &DhcpV4Lease = unsafe { &*lease };
    lease.gateways.as_ref().map(|g| g.len()).unwrap_or_default()
}

#[no_mangle]
pub extern "C" fn mozim_dhcpv4_lease_get_gateway(
    lease: *const DhcpV4Lease,
    index: usize,
) -> u32 {
    let lease: &DhcpV4Lease = unsafe { &*lease };
    lease
        .gateways
        .as_ref()
        .and_then(|g| g.get(index))
        .map(|i| u32::from(*i))
        .unwrap_or_default()
        .to_be()
}

#[no_mangle]
pub extern "C" fn mozim_dhcpv4_lease_get_yiaddr(
    lease: *const DhcpV4Lease,
) -> u32 {
    let lease: &DhcpV4Lease = unsafe { &*lease };
    u32::from(lease.yiaddr).to_be()
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
