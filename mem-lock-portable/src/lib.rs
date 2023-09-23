#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg, doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]
//------------------------------------------------------------------------
// Documentation:
//------------------------------------------------------------------------
#![doc(
    html_logo_url = "https://cdn.jsdelivr.net/gh/pinkforest/mem-protect/docs/assets/mem-protect-logo-clear.png"
)]
#![doc = include_str!("../README.md")]
//------------------------------------------------------------------------
// Linting:
//------------------------------------------------------------------------
#![cfg_attr(allow_unused_unsafe, allow(unused_unsafe))]
#![deny(
    clippy::undocumented_unsafe_blocks,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

// TODO: What we should do between raw/low level vs higher level abstractions ?

//------------------------------------------------------------------------
// Select implementation to which to abstract and expose over
//------------------------------------------------------------------------

#[cfg(feature = "mlock")]
mod mlock;
#[cfg(feature = "mlock")]
pub use mlock::*;
