//! Select implementation details to easen up gating in code
//!
//! This build script will always default to bypass with warn
//! upon undetected platform / failure in determinining which
//! implementation/s / configuration/s are to be used.
//!
//! **Default** Unsupported Platform: Warning**
//!
//! If this crate does not either recognise the target platform or
//! the target platform is not supported for mem-protect, we raise
//! a warning to the effect by default.
//!
//! **Build Configuration Options**
//!
//! These configuration options are provided for the top level crates to
//! set as desired protection level.
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
static ERR_MSG_NO_TARGET: &str = "Standard Cargo TARGET environment variable is not set";

// Custom Non-Rust standard target platforms require explicit settings.
static ERR_MSG_NO_PLATFORM: &str = "Unknown Rust target platform.";

// -------------------------------------------------------------------------
// effects as documented at top
// -------------------------------------------------------------------------
#[derive(Debug, Default)]
enum BuildEffects {
    Strict,
    Silent,
    #[default]
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
    
    // Determine target triplet string
    // This can fail in build environment that does not expose TARGET
    let target = match std::env::var("TARGET") {
        Ok(t) => Some(t),
        Err(_) => {
            raise_build_effect(&build_effects, ERR_MSG_NO_TARGET);
            None
        },
    };

    // Determine programmatic target platform
    // This fails if the target is custom non-rust
    let platform = if let Some(t) = target {
        match platforms::Platform::find(&t) {
            Some(p) => Some(p),
            None => {
                raise_build_effect(&build_effects, ERR_MSG_NO_PLATFORM);
                None
            }
        }
    }
    else {
        None
    };

    // TODO
    let backend: &'static str = "libc";
    
    println!("cargo:rustc-cfg=backend=\"{backend}\"");
}

// Raise determined warn or error build effect given desired configuration
fn raise_build_effect(build_effect: &BuildEffects, effect_msg: &'static str) {
    match build_effect {
        // Strict: Panic out
        BuildEffects::Strict => panic!("{}", effect_msg),
        // BestEffort: Raise build warning but let the build continue best effort
        BuildEffects::WarnAndContinue => println!("cargo:warning=\"{}\"", effect_msg),
        // Silent: Be quiet.
        BuildEffects::Silent => {},
    };
}
