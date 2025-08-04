use crate::error::{KernelError, KernelResult};
use crate::mem::{alloc, free, GFP_KERNEL};
use core::alloc::Layout;
use core::marker::PhantomData;
use core::ptr::{self, NonNull};

pub struct KVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> KVec<T> {
    pub fn new() -> Self {
        KVec {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, value: T) -> KernelResult<()> {
        if self.len == self.cap {
            self.grow()?;
        }

        unsafe {
            let end = self.ptr.as_ptr().add(self.len);
            ptr::write(end, value);
            self.len += 1;
        }

        Ok(())
    }

    fn grow(&mut self) -> KernelResult<()> {
        let new_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        let old_layout = Layout::array::<T>(self.cap).unwrap();

        let new_ptr = unsafe {
            let ptr = alloc(new_layout.size() as u64, GFP_KERNEL);
            if ptr.is_null() {
                return Err(KernelError::ENOMEM);
            }
            if self.cap > 0 {
                ptr::copy_nonoverlapping(
                    self.ptr.as_ptr() as *mut u8,
                    ptr,
                    old_layout.size(),
                );
                free(self.ptr.as_ptr() as *mut u8);
            }
            ptr
        };

        self.ptr = unsafe { NonNull::new_unchecked(new_ptr as *mut T) };
        self.cap = new_cap;
        Ok(())
    }
}

impl<T> Drop for KVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            while self.len > 0 {
                self.len -= 1;
                unsafe {
                    let ptr = self.ptr.as_ptr().add(self.len);
                    ptr::drop_in_place(ptr);
                }
            }
            unsafe {
                free(self.ptr.as_ptr() as *mut u8);
            }
        }
    }
}