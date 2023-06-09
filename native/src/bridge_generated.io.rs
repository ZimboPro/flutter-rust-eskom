use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_platform(port_: i64) {
    wire_platform_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_rust_release_mode(port_: i64) {
    wire_rust_release_mode_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_tick(port_: i64) {
    wire_tick_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_api_key(port_: i64, api_key: *mut wire_uint_8_list) {
    wire_test_api_key_impl(port_, api_key)
}

#[no_mangle]
pub extern "C" fn wire_set_api_key(port_: i64, api_key: *mut wire_uint_8_list) {
    wire_set_api_key_impl(port_, api_key)
}

#[no_mangle]
pub extern "C" fn wire_set_cache_dir(port_: i64, cache_dir: *mut wire_uint_8_list) {
    wire_set_cache_dir_impl(port_, cache_dir)
}

#[no_mangle]
pub extern "C" fn wire_allowance(port_: i64, api_key: *mut wire_uint_8_list) {
    wire_allowance_impl(port_, api_key)
}

#[no_mangle]
pub extern "C" fn wire_area_search(port_: i64, search_term: *mut wire_uint_8_list) {
    wire_area_search_impl(port_, search_term)
}

#[no_mangle]
pub extern "C" fn wire_area_info(port_: i64, area_id: *mut wire_uint_8_list) {
    wire_area_info_impl(port_, area_id)
}

#[no_mangle]
pub extern "C" fn wire_add_area(port_: i64, area_id: *mut wire_uint_8_list) {
    wire_add_area_impl(port_, area_id)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
