use crate::bindings;
use crate::error::{KernelError, KernelResult};
use crate::mem::{GFP_KERNEL, GfpFlags};
use core::alloc::Layout;
use core::ffi::c_ulong;
use core::ptr::NonNull;

pub unsafe trait KernelAllocator {
    fn alloc(&self, layout: Layout, flags: GfpFlags) -> KernelResult<NonNull<u8>>;
    fn dealloc(&self, ptr: NonNull<u8>);

    fn alloc_zeroed(&self, layout: Layout, flags: GfpFlags) -> KernelResult<NonNull<u8>> {
        let ptr = self.alloc(layout, flags)?;
        unsafe {
            ptr.as_ptr().write_bytes(0, layout.size());
        }
        Ok(ptr)
    }
}

#[derive(Copy, Clone)]
pub struct GlobalAllocator;

pub static KERNEL_ALLOC: GlobalAllocator = GlobalAllocator;

unsafe impl KernelAllocator for GlobalAllocator {
    fn alloc(&self, layout: Layout, flags: GfpFlags) -> KernelResult<NonNull<u8>> {
        let ptr = unsafe {
            bindings::kmalloc(layout.size() as c_ulong, flags) as *mut u8
        };
        NonNull::new(ptr).ok_or(KernelError::ENOMEM)
    }

    fn dealloc(&self, ptr: NonNull<u8>) {
        unsafe { bindings::kfree(ptr.as_ptr() as *const _); }
    }

    fn alloc_zeroed(&self, layout: Layout, flags: GfpFlags) -> KernelResult<NonNull<u8>> {
        let ptr = unsafe {
            bindings::kzalloc(layout.size() as c_ulong, flags) as *mut u8
        };
        NonNull::new(ptr).ok_or(KernelError::ENOMEM)
    }
}