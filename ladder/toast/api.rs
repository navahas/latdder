use std::marker::PhantomData;

pub struct PluggedIn;
pub struct Unplugged;
pub struct NoBread;
pub struct Bread;

pub struct Toaster<Power, Bread>(PhantomData<(Power, Bread)>);

impl Default for Toaster<Unplugged, NoBread> {
    fn default() -> Self {
        Self::new()
    }
}

impl Toaster<Unplugged, NoBread> {
    pub fn new() -> Self {
        Toaster(PhantomData)
    }

    pub fn plug_in(self) -> Toaster<PluggedIn, NoBread> {
        Toaster(PhantomData)
    }
}

impl<Power> Toaster<Power, NoBread> {
    pub fn insert_bread(self) -> Toaster<Power, Bread> {
        Toaster(PhantomData)
    }
}

impl Toaster<PluggedIn, Bread> {
    pub fn toast(self) -> Self {
        self
    }
}
