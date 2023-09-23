//! Memory locking abstraction

/// TODO: DOC
pub trait MemLockable {
    /// TODO: DOC
    unsafe fn lock<T: Sized>(_t: &T);
    /// TODO: DOC
    unsafe fn unlock<T: Sized>(_t: &T);
}

/// TODO: DOC
pub trait MemLockableSlice {
    /// TODO: DOCK
    unsafe fn lock_slice<T: Sized>(_t: &[T]);
    /// TODO: DOCK
    unsafe fn unlock_slice<T: Sized>(_t: &[T]);
}

// TODO: What we should do between raw/low level vs higher level abstractions ?

//------------------------------------------------------------------------
// Select implementation to which to abstract and expose over
//------------------------------------------------------------------------

#[cfg(mem_protect_methods = "libc_mlock")]
mod libc_crate;
#[cfg(mem_protect_methods = "libc_mlock")]
pub use libc_crate::*;

#[cfg(mem_protect_methods = "winapi_mlock")]
mod winapi_crate;
#[cfg(mem_protect_methods = "winapi_mlock")]
pub use winapi_crate::*;

// TODO: check if mem_protect_build_effects = "strict" and compile_error in case this is hit
#[cfg(all(
    not(mem_protect_methods = "libc_mlock"),
    not(mem_protect_methods = "winapi_mlock")
))]
mod dummy;
#[cfg(all(
    not(mem_protect_methods = "libc_mlock"),
    not(mem_protect_methods = "winapi_mlock")
))]
pub use dummy::*;
