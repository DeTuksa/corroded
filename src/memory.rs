use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

pub struct Dangling<T> {
    ptr: *mut T,
    layout: Layout,
}

impl<T> Dangling<T> {
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        unsafe {
            let ptr = alloc(layout) as *mut T;
            ptr.write(value);
            dealloc(ptr as *mut u8, layout);
            Dangling { ptr, layout }
        }
    }

    pub fn read(&self) -> T
    where
        T: Copy,
    {
        unsafe { ptr::read(self.ptr) }
    }

    pub fn write(&self, value: T) {
        unsafe {
            ptr::write(self.ptr, value);
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}

unsafe impl<T> Send for Dangling<T> {}
unsafe impl<T> Sync for Dangling<T> {}

pub fn double_free<T>(val: T) {
    let boxed = Box::new(val);
    let ptr = Box::into_raw(boxed);
    unsafe {
        drop(Box::from_raw(ptr));
        drop(Box::from_raw(ptr));
    }
}

pub fn leak<T>(val: T) -> *mut T {
    Box::into_raw(Box::new(val))
}

pub fn leak_ref<T>(val: T) -> &'static mut T {
    Box::leak(Box::new(val))
}

pub fn read_freed<T: Copy>(ptr: *mut T) -> T {
    unsafe { ptr::read(ptr) }
}

pub fn write_freed<T>(ptr: *mut T, val: T) {
    unsafe {
        ptr::write(ptr, val);
    }
}

pub fn alloc_garbage<T>() -> *mut T {
    let layout = Layout::new::<T>();
    unsafe { alloc(layout) as *mut T }
}

pub fn free<T>(ptr: *mut T) {
    let layout = Layout::new::<T>();
    unsafe {
        dealloc(ptr as *mut u8, layout);
    }
}

pub struct ArbitraryAccess;

impl ArbitraryAccess {
    pub fn read<T: Copy>(addr: usize) -> T {
        unsafe { ptr::read(addr as *const T) }
    }

    pub fn write<T>(addr: usize, val: T) {
        unsafe {
            ptr::write(addr as *mut T, val);
        }
    }
}
