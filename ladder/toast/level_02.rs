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


// cargo test --test toast_level_02


use std::marker::PhantomData;

pub struct PluggedIn;
pub struct Unplugged;
pub struct BreadIn;
pub struct NoBread;

pub struct Toaster<Power, Bread>(PhantomData<(Power, Bread)>);

impl Toaster<Unplugged, NoBread> {
    pub fn new() -> Self {
        Toaster(PhantomData)
    }

    pub fn plug_in(self) -> Toaster<PluggedIn, NoBread> {
        Toaster(PhantomData)
    }
}

impl Toaster<PluggedIn, NoBread> {
    pub fn insert_bread(self) -> Toaster<PluggedIn, BreadIn> {
        Toaster(PhantomData)
    }
}

impl Toaster<PluggedIn, BreadIn> {
    pub fn toast(self) -> Self {
        self
    }
}
