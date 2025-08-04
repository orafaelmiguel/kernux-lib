use crate::bindings;
use core::ffi::{c_void, c_ulong};
use core::ptr;

pub mod kbox;
pub mod kvec;

pub use kbox::KBox;
pub use kvec::KVec;

pub type GfpFlags = c_ulong;

pub const GFP_KERNEL: GfpFlags = 0xCC0;
pub const GFP_ATOMIC: GfpFlags = 0x80;

pub(crate) unsafe fn alloc(size: c_ulong, flags: GfpFlags) -> *mut u8 {
    bindings::kmalloc(size, flags) as *mut u8
}

pub(crate) unsafe fn alloc_zeroed(size: c_ulong, flags: GfpFlags) -> *mut u8 {
    bindings::kzalloc(size, flags) as *mut u8
}

pub(crate) unsafe fn free(ptr: *mut u8) {
    if !ptr.is_null() {
        bindings::kfree(ptr as *const c_void);
    }
}