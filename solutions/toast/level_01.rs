use std::marker::PhantomData;

#[derive(Debug)]
pub struct PluggedIn;

#[derive(Debug)]
pub struct Unplugged;

#[derive(Debug)]
pub struct Toaster<State>(PhantomData<State>);

impl Toaster<Unplugged> {
    pub fn new() -> Self {
        Toaster(PhantomData)
    }

    pub fn plug_in(self) -> Toaster<PluggedIn> {
        println!("🔌 Toaster plugged in.");
        Toaster(PhantomData)
    }
}

impl Toaster<PluggedIn> {
    pub fn toast(self) -> Self {
        println!("🍞 Toasting...");
        println!("🍳 Toast ready!");
        self
    }
}
