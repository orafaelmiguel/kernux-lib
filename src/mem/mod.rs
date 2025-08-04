pub mod alloc;
pub mod kbox;
pub mod kvec;

pub use alloc::{GlobalAllocator, KernelAllocator, KERNEL_ALLOC};
pub use kbox::KBox;
pub use kvec::KVec;

use core::ffi::c_ulong;
pub type GfpFlags = c_ulong;
pub const GFP_KERNEL: GfpFlags = 0xCC0;
pub const GFP_ATOMIC: GfpFlags = 0x80;