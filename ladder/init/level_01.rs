use std::marker::PhantomData;

pub struct Ready;
pub struct Latdder<State>(PhantomData<State>);

impl Latdder<Ready> {
    pub fn start() -> Self {
        Latdder(PhantomData)
    }
}
