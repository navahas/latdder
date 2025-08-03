// Level 4 – Ejecting Toast
//
// In this level, you can only call `.eject()` **after** `.toast()`.
// This introduces a new state: `Toasted`, which must be present before `eject()` is available.
//
// Teaches: multiple chained state transitions, enforcing strict action order
// fn valid_eject_sequence() {
//     let _ = Toaster::<Unplugged, NoBread>::new()
//         .plug_in()
//         .insert_bread()
//         .set_timer(3)
//         .toast()
//         .eject();
// }

use std::{marker::PhantomData, thread, time::Duration};

// Power states
pub struct PluggedIn;
pub struct Unplugged;
// Bread states
pub struct BreadIn;
pub struct NoBread;
pub struct Toasted;
pub struct Ejected;

pub struct Toaster<Power, Bread>(PhantomData<(Power, Bread)>);

impl Toaster<Unplugged, NoBread> {
    pub fn new() -> Self {
        Toaster(PhantomData)
    }
}

impl<Bread> Toaster<Unplugged, Bread> {
    pub fn plug_in(self) -> Toaster<PluggedIn, Bread> {
        Toaster(PhantomData)
    }
}

impl<Power> Toaster<Power, NoBread> {
    pub fn insert_bread(self) -> Toaster<Power, BreadIn> {
        Toaster(PhantomData)
    }
}

impl Toaster<PluggedIn, BreadIn> {
    pub fn set_timer(self, seconds: u8) -> Self {
        thread::sleep(Duration::from_secs(seconds as u64));
        self
    }
}

impl Toaster<PluggedIn, BreadIn> {
    pub fn toast(self) -> Toaster<PluggedIn, Toasted> {
        Toaster(PhantomData)
    }
}

impl Toaster<PluggedIn, Toasted> {
    pub fn eject(self) -> Toaster<PluggedIn, Ejected> {
        Toaster(PhantomData)
    }
}
