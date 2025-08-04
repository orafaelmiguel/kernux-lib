use crate::error::{KernelError, KernelResult};
use crate::mem::{alloc, free, GFP_KERNEL};
use core::alloc::Layout;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::ptr::{self, NonNull};
use core::slice;

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

    pub fn len(&self) -> usize {
        self.len
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

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
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
                ptr::copy_nonoverlapping(self.ptr.as_ptr() as *mut u8, ptr, old_layout.size());
                free(self.ptr.as_ptr() as *mut u8);
            }
            ptr
        };

        self.ptr = unsafe { NonNull::new_unchecked(new_ptr as *mut T) };
        self.cap = new_cap;
        Ok(())
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> Drop for KVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            while let Some(_) = self.pop() {}
            unsafe {
                free(self.ptr.as_ptr() as *mut u8);
            }
        }
    }
}

impl<T> Index<usize> for KVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T> IndexMut<usize> for KVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}


#[cfg(test)]
mod tests {
    use super::KVec;

    #[test]
    fn kvec_new_is_empty() {
        let vec = KVec::<u32>::new();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn kvec_push_and_pop() {
        let mut vec = KVec::new();
        vec.push(10).unwrap();
        vec.push(20).unwrap();
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 10);
        assert_eq!(vec[1], 20);

        assert_eq!(vec.pop(), Some(20));
        assert_eq!(vec.len(), 1);

        assert_eq!(vec.pop(), Some(10));
        assert_eq!(vec.len(), 0);

        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn kvec_growth_and_reallocation() {
        let mut vec = KVec::new();
        for i in 0..10 {
            vec.push(i).unwrap();
        }
        assert_eq!(vec.len(), 10);
        assert_eq!(vec[0], 0);
        assert_eq!(vec[5], 5);
        assert_eq!(vec[9], 9);
    }

    #[test]
    fn kvec_index_mut() {
        let mut vec = KVec::new();
        vec.push(100).unwrap();
        vec[0] += 5;
        assert_eq!(vec[0], 105);
    }

    #[test]
    fn kvec_iter() {
        let mut vec = KVec::new();
        vec.push(1).unwrap();
        vec.push(2).unwrap();
        vec.push(3).unwrap();

        let mut sum = 0;
        for item in vec.iter() {
            sum += item;
        }
        assert_eq!(sum, 6);
    }
}