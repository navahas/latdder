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
