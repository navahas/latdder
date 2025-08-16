//! # Toast — Level 4: Toast Ejection
//! GOAL: Only allow ejecting after toasting by introducing a Toasted state.
//!
//! ## ✅ Works
//! ```rust
//! use ladder::toast::api::*;
//! let _ = Toaster::<Unplugged, NoBread>::new()
//!     .plug_in()
//!     .insert_bread()
//!     .set_timer(3)
//!     .toast()
//!     .eject();
//! ```
//!
//! ## ❌ Fails to compile
//! ```compile_fail
//! use ladder::toast::api::*;
//! let _ = Toaster::<PluggedIn, Bread>::new()
//!     .eject(); // Cannot eject before toasting
//! ```

#![cfg(feature = "toast_level_04")]
