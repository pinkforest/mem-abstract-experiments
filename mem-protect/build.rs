//! Select implementation details to easen up gating in code
//!
//! **Default** Unsupported Platform: Warning**
//!
//! If this crate does not either recognise the target platform or
//! the target platform is not supported for mem-protect, we raise
//! a build related warning to the effect by default.
//!
//! **Build Configuration Options**
//!
//! These configuration options are provided for the top level crates to
//! set as desired be-informed-at level.
//!
//! cfg(mem_protect_build_effects = "strict")
//! Optionally can be used to panic the build in case any of the desired
//! implementation/s or were not be able to be set / enforced.
//!
//! cfg(mem_protect_build_effects = "silent")
//! Optionally can be used to be silent if implementation could not be used.
//!

// -------------------------------------------------------------------------
// Hygiene
// -------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![warn(
    clippy::unwrap_used,
    missing_docs,
    dead_code,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

// -------------------------------------------------------------------------
// The below warning / error messages may be given
// -------------------------------------------------------------------------

// Standard Cargo TARGET environment variable of triplet is required
static ERR_MSG_NO_TARGET_FAMILY: &str =
    "Standard CARGO_CFG_TARGET_FAMILY environment variable is not set";

// -------------------------------------------------------------------------
// effects as documented at top
// -------------------------------------------------------------------------
#[derive(Debug)]
enum BuildEffects {
    Strict,
    Silent,
    WarnAndContinue,
}

fn main() {
    // See BuildEffects
    let build_effects = match std::env::var("CARGO_CFG_MEM_BUILD_EFFECTS").as_deref() {
        // Opt-in this acts strictly
        Ok("strict") => BuildEffects::Strict,
        // Opt-in this acts silently as best effort
        Ok("silent") => BuildEffects::Silent,
        // By default this is best effort with warnings
        _ => BuildEffects::WarnAndContinue,
    };

    // are we unix or windows ?
    let family_methods = match std::env::var("CARGO_CFG_TARGET_FAMILY").as_deref() {
        Ok("unix") => Some("libc_mlock"),
        Ok("windows") => Some("winapi_mlock"),
        _ => {
            raise_build_effect(&build_effects, ERR_MSG_NO_TARGET_FAMILY);
            None
        }
    };

    if let Some(method_add) = family_methods {
        println!("cargo:rustc-cfg=mem_protect_methods=\"{method_add}\"");
    }
}

// Raise determined warn or error build effect given desired configuration
fn raise_build_effect(build_effect: &BuildEffects, effect_msg: &'static str) {
    match build_effect {
        // Strict: Panic out
        BuildEffects::Strict => panic!("{}", effect_msg),
        // BestEffort: Raise build warning but let the build continue best effort
        BuildEffects::WarnAndContinue => println!("cargo:warning=\"{}\"", effect_msg),
        // Silent: Be quiet.
        BuildEffects::Silent => {}
    };
}
