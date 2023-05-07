// SPDX-License-Identifier: Apache-2.0

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
