use std::{marker::PhantomData, thread, time::Duration};

pub struct PluggedIn;
pub struct Unplugged;
pub struct BreadIn;
pub struct NoBread;
pub struct Toasted;
pub struct Ejected;
pub struct NoTimer;
pub struct TimerSet;

// Event enum for runtime actions
pub enum Event {
    PlugIn,
    InsertBread,
    SetTimer(u8),
    Toast,
    Eject,
}

pub struct Toaster<Power, Bread, Timer> {
    seconds: Option<u8>,
    _power: PhantomData<Power>,
    _bread: PhantomData<Bread>,
    _timer: PhantomData<Timer>,
}

impl Toaster<Unplugged, NoBread, NoTimer> {
    pub fn new() -> Self {
        Self {
            seconds: None,
            _power: PhantomData,
            _bread: PhantomData,
            _timer: PhantomData,
        }
    }

    pub fn handle_event(self, event: Event) -> Result<Toaster<PluggedIn, NoBread, NoTimer>, &'static str> {
        match event {
            Event::PlugIn => Ok(Toaster {
                seconds: self.seconds,
                _power: PhantomData,
                _bread: PhantomData,
                _timer: PhantomData,
            }),
            _ => Err("Invalid event for current state"),
        }
    }
}

impl<Bread> Toaster<Unplugged, Bread, NoTimer> {
    pub fn plug_in(self) -> Toaster<PluggedIn, Bread, NoTimer> {
        Toaster {
            seconds: self.seconds,
            _power: PhantomData,
            _bread: PhantomData,
            _timer: PhantomData,
        }
    }
}

impl<Power> Toaster<Power, NoBread, NoTimer> {
    pub fn insert_bread(self) -> Toaster<Power, BreadIn, NoTimer> {
        Toaster {
            seconds: self.seconds,
            _power: PhantomData,
            _bread: PhantomData,
            _timer: PhantomData,
        }
    }
}

impl Toaster<PluggedIn, BreadIn, NoTimer> {
    pub fn set_timer(self, seconds: u8) -> Toaster<PluggedIn, BreadIn, TimerSet> {
        Toaster {
            seconds: Some(seconds),
            _power: PhantomData,
            _bread: PhantomData,
            _timer: PhantomData,
        }
    }
}

impl Toaster<PluggedIn, BreadIn, TimerSet> {
    pub fn toast(self) -> Toaster<PluggedIn, Toasted, TimerSet> {
        if let Some(secs) = self.seconds {
            thread::sleep(Duration::from_secs(secs as u64));
        }
        Toaster {
            seconds: self.seconds,
            _power: PhantomData,
            _bread: PhantomData,
            _timer: PhantomData,
        }
    }
}

impl Toaster<PluggedIn, Toasted, TimerSet> {
    pub fn eject(self) -> Toaster<PluggedIn, Ejected, NoTimer> {
        Toaster {
            seconds: None,
            _power: PhantomData,
            _bread: PhantomData,
            _timer: PhantomData,
        }
    }
}

fn main() {
    let _ = Toaster::new()
        .handle_event(Event::PlugIn).unwrap()
        .insert_bread()
        .set_timer(1)
        .toast()
        .eject();

    println!("Toaster is ready!");
}
