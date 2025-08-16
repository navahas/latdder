//! # Toast — Level 1: Basic Typestate
//! GOAL: Prevent toasting when unplugged using compile-time state tracking.
//!
//! ## ✅ Works
//! ```rust
//! use ladder::toast::api::*;
//! let _ = Toaster::<Unplugged, NoBread>::new()
//!     .plug_in()
//!     .toast();
//! ```
//!
//! ## ❌ Fails to compile
//! ```compile_fail
//! use ladder::toast::api::*;
//! let _ = Toaster::<Unplugged, NoBread>::new()
//!     .toast(); // Cannot toast while unplugged
//! ```

#![cfg(feature = "toast_level_01")]
