// Level 2 – Insert Bread
//
// In this level, toasting is only allowed **after both:**
// - The toaster is plugged in
// - Bread has been inserted
//
// Try to call `.toast()` too early and Rust will stop you!
//         let _ = Toaster::<Unplugged, NoBread>::new()
//            .plug_in()
//            .insert_bread()
//            .toast();
#![cfg(feature = "toast_level_02")]

use crate::toast::api::*;

pub fn target() {
    let _ = Toaster::<Unplugged, NoBread>::new()
        .plug_in()
        .insert_bread()
        .toast();
}
