use crate::error::{KernelError, KernelResult};
use crate::mem::alloc::{KernelAllocator, KERNEL_ALLOC};
use crate::mem::GFP_KERNEL;
use core::alloc::Layout;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::ptr::{self, NonNull};
use core::slice;

pub struct KVec<T, A: KernelAllocator = GlobalAllocator> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    allocator: A,
    _marker: PhantomData<T>,
}

impl<T> KVec<T, GlobalAllocator> {
    pub fn new() -> Self {
        Self::new_in(KERNEL_ALLOC)
    }
}

impl<T, A: KernelAllocator> KVec<T, A> {
    pub fn new_in(allocator: A) -> Self {
        KVec {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
            allocator,
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, value: T) -> KernelResult<()> {
        if self.len == self.cap {
            self.grow()?;
        }
        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), value);
            self.len += 1;
        }
        Ok(())
    }

    fn grow(&mut self) -> KernelResult<()> {
        let new_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
        let new_layout = Layout::array::<T>(new_cap).unwrap();

        let new_ptr = self.allocator.alloc(new_layout, GFP_KERNEL)?;

        if self.cap > 0 {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                ptr::copy_nonoverlapping(self.ptr.as_ptr(), new_ptr.as_ptr() as *mut T, self.len);
                self.allocator.dealloc(self.ptr.cast());
            }
        }
        self.ptr = new_ptr.cast();
        self.cap = new_cap;
        Ok(())
    }
}