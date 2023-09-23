//! libc mlock

use super::*;
use core::mem;

// Dummy types.
struct MemLocking;

impl MemLockable for MemLocking {
    unsafe fn lock<T: Sized>(m: &T) {
        let ptr: *const T = m;
        let size = mem::size_of::<T>();

        // SAFETY: TODO
        unsafe {
            libc::mlock(ptr as *mut libc::c_void, size);
            #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
            #[no_coverage]
            libc::madvise(ptr, size, libc::MADV_NOCORE);
            #[cfg(target_os = "linux")]
            libc::madvise(ptr as *mut libc::c_void, size, libc::MADV_DONTDUMP);
        }
    }
    unsafe fn unlock<T: Sized>(m: &T) {
        let ptr: *const T = m;
        let size = mem::size_of::<T>();

        // SAFETY: TODO
        unsafe {
            libc::munlock(ptr as *mut libc::c_void, size);
            #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
            #[no_coverage]
            libc::madvise(ptr, size, libc::MADV_CORE);
            #[cfg(target_os = "linux")]
            libc::madvise(ptr as *mut libc::c_void, size, libc::MADV_DODUMP);
        }
    }
}

impl MemLockableSlice for MemLocking {
    unsafe fn lock_slice<T: Sized>(m: &[T]) {
        let size = mem::size_of_val(m);
        let ptr = m.as_ptr() as *mut libc::c_void;

        // SAFETY: TODO
        unsafe {
            libc::mlock(ptr, size);

            #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
            #[no_coverage]
            libc::madvise(ptr, size, libc::MADV_NOCORE);

            #[cfg(target_os = "linux")]
            libc::madvise(ptr, size, libc::MADV_DONTDUMP);
        }
    }
    unsafe fn unlock_slice<T: Sized>(m: &[T]) {
        let size = mem::size_of_val(m);
        let ptr = m.as_ptr() as *mut libc::c_void;

        // SAFETY: TODO
        unsafe {
            libc::munlock(ptr, size);

            #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
            #[no_coverage]
            libc::madvise(ptr, size, libc::MADV_CORE);

            #[cfg(target_os = "linux")]
            libc::madvise(ptr, size, libc::MADV_DODUMP);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_u8() {
        let a: u8 = 0;
        unsafe {
            MemLocking::lock(&a);
        }
    }

    #[test]
    fn test_lock_unit() {
        let a = ();
        assert_eq!(0, mem::size_of_val(&a));
        unsafe {
            MemLocking::lock(&a);
        }
    }

    #[test]
    fn test_unlock_u8() {
        let a: u8 = 0;
        unsafe {
            MemLocking::unlock(&a);
        }
    }

    #[test]
    fn test_lock_slice() {
        let a: [u8; 2] = [0, 0];
        unsafe {
            MemLocking::lock_slice(&a);
        }
    }
    #[test]
    fn test_unlock_slice() {
        let a: [u8; 2] = [0, 0];
        unsafe {
            MemLocking::unlock_slice(&a);
        }
    }
}
