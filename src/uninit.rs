use std::mem::MaybeUninit;

pub fn garbage<T>() -> T {
    unsafe { MaybeUninit::<T>::uninit().assume_init() }
}

pub fn garbage_array<T, const N: usize>() -> [T; N] {
    unsafe { MaybeUninit::<[T; N]>::uninit().assume_init() }
}

pub fn garbage_vec<T>(len: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(len);
    unsafe {
        v.set_len(len);
    }
    v
}

pub struct CursedMaybeUninit<T> {
    inner: MaybeUninit<T>,
}

impl<T> CursedMaybeUninit<T> {
    pub fn uninit() -> Self {
        CursedMaybeUninit {
            inner: MaybeUninit::uninit(),
        }
    }

    pub fn new(val: T) -> Self {
        CursedMaybeUninit {
            inner: MaybeUninit::new(val),
        }
    }

    pub fn assume_init(self) -> T {
        unsafe { self.inner.assume_init() }
    }

    pub fn assume_init_ref(&self) -> &T {
        unsafe { self.inner.assume_init_ref() }
    }

    pub fn assume_init_mut(&mut self) -> &mut T {
        unsafe { self.inner.assume_init_mut() }
    }

    pub fn write(&mut self, val: T) {
        self.inner.write(val);
    }

    pub fn into_inner(self) -> MaybeUninit<T> {
        self.inner
    }
}

impl<T: Copy> CursedMaybeUninit<T> {
    pub fn read(&self) -> T {
        unsafe { self.inner.assume_init_read() }
    }
}

pub fn zeroed<T>() -> T {
    unsafe { MaybeUninit::<T>::zeroed().assume_init() }
}

pub fn fill_garbage<T>(slice: &mut [T]) {
    for elem in slice.iter_mut() {
        unsafe {
            let garbage = MaybeUninit::<T>::uninit().assume_init();
            std::ptr::write(elem, garbage);
        }
    }
}

pub fn forget<T>(val: T) {
    std::mem::forget(val);
}

pub fn garbage_box<T>() -> Box<T> {
    unsafe {
        let ptr = std::alloc::alloc(std::alloc::Layout::new::<T>()) as *mut T;
        Box::from_raw(ptr)
    }
}

pub fn read_padding<T>(val: &T) -> Vec<u8> {
    let size = std::mem::size_of::<T>();
    let ptr = val as *const T as *const u8;
    let mut bytes = Vec::with_capacity(size);
    for i in 0..size {
        unsafe {
            bytes.push(*ptr.add(i));
        }
    }
    bytes
}
