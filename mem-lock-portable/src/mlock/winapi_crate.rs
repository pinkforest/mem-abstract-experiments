//! libc mlock

use super::*;
use core::mem;

// Dummy types.
struct MemLocking;

impl MemLockable for MemLocking {
    unsafe fn lock<T: Sized>(m: &T) {
        let addr: *const T = m;
        let size = mem::size_of_val(m);
        // SAFETY: TODO
        unsafe {
            ::winapi::um::memoryapi::VirtualLock(
                addr as ::winapi::shared::minwindef::LPVOID,
                size as ::winapi::shared::basetsd::SIZE_T,
            );
        }
    }
    unsafe fn unlock<T: Sized>(m: &T) {
        let addr: *const T = m;
        let size = mem::size_of_val(m);
        // SAFETY: TODO
        unsafe {
            ::winapi::um::memoryapi::VirtualUnlock(
                addr as ::winapi::shared::minwindef::LPVOID,
                size as ::winapi::shared::basetsd::SIZE_T,
            );
        }
    }
}

impl MemLockableSlice for MemLocking {
    unsafe fn lock_slice<T: Sized>(m: &[T]) {
        // SAFETY: TODO
        unsafe {
            let addr = m.as_ptr() as ::winapi::shared::minwindef::LPVOID;
            let size = mem::size_of_val(m);
            ::winapi::um::memoryapi::VirtualLock(addr, size as ::winapi::shared::basetsd::SIZE_T);
        }
    }
    unsafe fn unlock_slice<T: Sized>(m: &[T]) {
        // SAFETY: TODO
        unsafe {
            let addr = m.as_ptr() as ::winapi::shared::minwindef::LPVOID;
            let size = mem::size_of_val(m);
            ::winapi::um::memoryapi::VirtualUnlock(
                addr as ::winapi::shared::minwindef::LPVOID,
                size as ::winapi::shared::basetsd::SIZE_T,
            );
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
