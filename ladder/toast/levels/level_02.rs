//! # Toast — Level 2: Insert Bread
//!
//! Toasting is allowed only after both:
//! - the toaster is **plugged in**
//! - the **bread is inserted**
//!
//! ## ✅ Correct
//! ```rust
//! use ladder::toast::api::*;
//! let _ = Toaster::<Unplugged, NoBread>::new()
//!     .plug_in()
//!     .insert_bread()
//!     .toast();
//! ```
//!
//! ## ❌ Incorrect (won’t compile)
//! ```compile_fail
//! use ladder::toast::api::*;
//! let _ = Toaster::<Unplugged, NoBread>::new()
//!     .plug_in()
//!     .toast(); // bread not inserted
//! ```
#[cfg_attr(docsrs, doc(cfg(feature = "toast_level_02")))]

use crate::toast::api::*;

pub fn target() {
    let _ = Toaster::<Unplugged, NoBread>::new()
        .plug_in()
        .insert_bread()
        .toast();
}
