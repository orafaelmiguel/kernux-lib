use crate::error::{KernelError, KernelResult};
use crate::mem::alloc::{KernelAllocator, KERNEL_ALLOC};
use crate::mem::GFP_KERNEL;
use core::alloc::Layout;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::ptr::{self, NonNull};

pub unsafe trait Zeroable {}

pub struct KBox<T, A: KernelAllocator = GlobalAllocator> {
    ptr: NonNull<T>,
    allocator: A,
    _marker: PhantomData<T>,
}

impl<T> KBox<T, GlobalAllocator> {
    pub fn new(value: T) -> KernelResult<Self> {
        Self::new_in(value, KERNEL_ALLOC)
    }
}

impl<T: Zeroable> KBox<T, GlobalAllocator> {
    pub fn new_zeroed() -> KernelResult<Self> {
        Self::new_zeroed_in(KERNEL_ALLOC)
    }
}

impl<T, A: KernelAllocator> KBox<T, A> {
    pub fn new_in(value: T, allocator: A) -> KernelResult<Self> {
        let layout = Layout::new::<T>();
        let ptr = allocator.alloc(layout, GFP_KERNEL)?;
        unsafe {
            ptr::write(ptr.as_ptr() as *mut T, value);
        }
        Ok(KBox {
            ptr: ptr.cast(),
            allocator,
            _marker: PhantomData,
        })
    }
}

impl<T: Zeroable, A: KernelAllocator> KBox<T, A> {
    pub fn new_zeroed_in(allocator: A) -> KernelResult<Self> {
        let layout = Layout::new::<T>();
        let ptr = allocator.alloc_zeroed(layout, GFP_KERNEL)?;
        Ok(KBox {
            ptr: ptr.cast(),
            allocator,
            _marker: PhantomData,
        })
    }
}

impl<T, A: KernelAllocator> Drop for KBox<T, A> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr());
            self.allocator.dealloc(self.ptr.cast());
        }
    }
}

impl<T, A: KernelAllocator> Deref for KBox<T, A> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T, A: KernelAllocator> DerefMut for KBox<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}