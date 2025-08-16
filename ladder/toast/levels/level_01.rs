//! # Toast — Level 1: Basic Typestate
//! GOAL: Prevent toasting when unplugged using compile-time state tracking.
//!
//! ## ✅ Works
//! ```rust
//! use latdder::toast::api::*;
//! let _ = Toaster::<Unplugged, NoBread>::new()
//!     .plug_in()
//!     .insert_bread()
//!     .toast();
//! ```
//!
//! ## ❌ Fails to compile
//! ```compile_fail
//! use latdder::toast::api::*;
//! let _ = Toaster::<Unplugged, NoBread>::new()
//!     .toast(); // Cannot toast while unplugged
//! ```

