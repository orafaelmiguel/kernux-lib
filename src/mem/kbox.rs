use crate::error::{KernelError, KernelResult};
use crate::mem::{alloc, free, GFP_KERNEL};
use core::alloc::Layout;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::ptr::{self, NonNull};

pub struct KBox<T> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T> KBox<T> {
    pub fn new(value: T) -> KernelResult<Self> {
        let layout = Layout::new::<T>();
        if layout.size() == 0 {
            return Ok(KBox {
                ptr: NonNull::dangling(),
                _marker: PhantomData,
            });
        }

        let raw_ptr = unsafe { alloc(layout.size() as u64, GFP_KERNEL) };

        if raw_ptr.is_null() {
            return Err(KernelError::ENOMEM);
        }

        let ptr = raw_ptr as *mut T;
        unsafe {
            ptr::write(ptr, value);
        }

        Ok(KBox {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
            _marker: PhantomData,
        })
    }
}

impl<T> Drop for KBox<T> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();
        if layout.size() > 0 {
            unsafe {
                ptr::drop_in_place(self.ptr.as_ptr());
                free(self.ptr.as_ptr() as *mut u8);
            }
        }
    }
}

impl<T> Deref for KBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for KBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}