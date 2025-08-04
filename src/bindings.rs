use core::ffi::{c_void, c_ulong};

#[link(name = "c")]
extern "C" {
    pub fn kmalloc(size: c_ulong, flags: c_ulong) -> *mut c_void;
    pub fn kzalloc(size: c_ulong, flags: c_ulong) -> *mut c_void;
    pub fn kfree(ptr: *const c_void);
}