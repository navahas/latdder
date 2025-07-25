// ## ⏱ Level 3 – Timer Setting with Trait Capability
//
// Add a `.set_timer()` method that is **only available once the toaster is plugged in**.
// This uses traits (`CanSetTimer`) to gate functionality by type.
//
// - `.set_timer()` should not compile if toaster is unplugged
// - `.toast()` still requires bread to be inserted
//
// Teaches: trait-based capability gating + type-state coordination
// let _ = Toaster::<Unplugged, NoBread>::new()
//     .plug_in()
//     .insert_bread()
//     .set_timer(10)
//     .toast();

use std::{marker::PhantomData, thread, time::Duration};

pub struct PluggedIn;
pub struct Unplugged;
pub struct BreadIn;
pub struct NoBread;

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

pub trait CanSetTimer {
    fn set_timer(self, seconds: u8) -> Self;
}

impl<Bread> CanSetTimer for Toaster<PluggedIn, Bread> {
    fn set_timer(self, seconds: u8) -> Self {
        thread::sleep(Duration::from_secs(seconds as u64));
        self
    }
}

impl Toaster<PluggedIn, BreadIn> {
    pub fn toast(self) -> Self {
        self
    }
}
