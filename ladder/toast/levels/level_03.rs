//! # Toast — Level 03: Trait-Gated Timer
//! GOAL: Add timer setting that only works when plugged in using trait capability gating.
//!
//! ## ✅ Works
//! ```rust
//! use ladder::toast::api::*;
//! let _ = Toaster::<Unplugged, NoBread>::new()
//!     .plug_in()
//!     .insert_bread()
//!     .set_timer(10)
//!     .toast();
//! ```
//!
//! ## ❌ Fails to compile
//! ```compile_fail
//! use ladder::toast::api::*;
//! let _ = Toaster::<Unplugged, NoBread>::new()
//!     .set_timer(10); // Cannot set timer while unplugged
//! ```

// #[cfg_attr(docsrs, doc(cfg(feature = "toast_level_03")))]
//
#![cfg(feature = "toast_level_03")]
