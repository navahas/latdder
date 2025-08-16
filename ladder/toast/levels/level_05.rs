//! # Toast — Level 5: Advanced State Management
//! GOAL: Implement complete toaster lifecycle with reset and multiple operation modes.
//!
//! ## ✅ Works
//! ```rust
//! use latdder::toast::api::*;
//! let _ = Toaster::<Unplugged, NoBread>::new()
//!     .plug_in()
//!     .insert_bread()
//!     .set_timer(5)
//!     .toast()
//!     .eject()
//!     .reset();
//! ```
//!
//! ## ❌ Fails to compile
//! ```compile_fail
//! use latdder::toast::api::*;
//! let _ = Toaster::<Unplugged, NoBread>::new()
//!     .reset(); // Cannot reset from initial state
//! ```

#![cfg(feature = "toast_level_05")]
