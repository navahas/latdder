use std::{marker::PhantomData, thread, time::Duration};


#[derive(Debug)]
pub struct PluggedIn;
#[derive(Debug)]
pub struct Unplugged;

#[derive(Debug)]
pub struct BreadIn;
#[derive(Debug)]
pub struct NoBread;
#[derive(Debug)]
pub struct Toasted;
#[derive(Debug)]
pub struct Ejected;

#[derive(Debug)]
pub struct NoTimer;
#[derive(Debug)]
pub struct TimerSet {
    _seconds: u8
}

#[derive(Debug)]
pub struct Toaster<Power, Bread, Timer>(PhantomData<(Power, Bread, Timer)>);

impl Toaster<Unplugged, NoBread, NoTimer> {
    pub fn new() -> Self {
        Toaster(PhantomData)
    }
}

impl<Bread> Toaster<Unplugged, Bread, NoTimer> {
    pub fn plug_in(self) -> Toaster<PluggedIn, Bread, NoTimer> {
        Toaster(PhantomData)
    }
}

impl<Power> Toaster<Power, NoBread, NoTimer> {
    pub fn insert_bread(self) -> Toaster<Power, BreadIn, NoTimer> {
        Toaster(PhantomData)
    }
}

impl Toaster<PluggedIn, BreadIn, NoTimer> {
    pub fn set_timer(self, seconds: u8) -> Toaster<PluggedIn, BreadIn, TimerSet> {
        thread::sleep(Duration::from_secs(seconds as u64));
        Toaster(PhantomData)
    }
}

impl Toaster<PluggedIn, BreadIn, TimerSet> {
    pub fn toast(self) -> Toaster<PluggedIn, Toasted, TimerSet> {
        Toaster(PhantomData)
    }
}

impl Toaster<PluggedIn, Toasted, TimerSet> {
    pub fn eject(self) -> Toaster<PluggedIn, Ejected, TimerSet> {
        Toaster(PhantomData)
    }
}

fn main() {

    let toaster = Toaster::new()
        .plug_in()
        .insert_bread()
        .set_timer(1)
        .toast()
        .eject();

    println!("{:?}", toaster);
}
