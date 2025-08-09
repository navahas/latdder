use std::marker::PhantomData;

pub struct PluggedIn;
pub struct Unplugged;

pub struct Toaster<State>(PhantomData<State>);

impl Toaster<Unplugged> {
    pub fn new() -> Self {
        Toaster(PhantomData)
    }

    pub fn plug_in(self) -> Toaster<PluggedIn> {
        Toaster(PhantomData)
    }
}

impl Toaster<PluggedIn> {
    pub fn toast(self) -> Self {
        self
    }
}
