//! libc mlock

use super::*;

// Dummy types.
struct MemLocking;

impl MemLockable for MemLocking {
    unsafe fn lock<T: Sized>(_: &T) {}
    unsafe fn unlock<T: Sized>(_: &T) {}
}

impl MemLockableSlice for MemLocking {
    unsafe fn lock_slice<T: Sized>(_: &[T]) {}
    unsafe fn unlock_slice<T: Sized>(_: &[T]) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_u8() {
        let a: u8 = 0;
        unsafe { MemLocking::lock(&a); }
    }

    #[test]
    fn test_unlock_u8() {
        let a: u8 = 0;
        unsafe { MemLocking::unlock(&a); }
    }

    #[test]
    fn test_lock_slice() {
        let a: [u8; 2] = [0, 0];
        unsafe { MemLocking::lock_slice(&a); }
    }
    #[test]
    fn test_unlock_slice() {
        let a: [u8; 2] = [0, 0];
        unsafe { MemLocking::unlock_slice(&a); }
    }
}
