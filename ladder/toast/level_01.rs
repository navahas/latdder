// TODO: Implement a type-safe toaster using phantom types
// 
// Requirements:
// - Create a Toaster<State> that prevents invalid operations
// - Implement PluggedIn and Unplugged states  
// - new() creates Unplugged toaster
// - plug_in() transitions from Unplugged to PluggedIn
// - toast() only works on PluggedIn toaster
//
// Your implementation here:

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
