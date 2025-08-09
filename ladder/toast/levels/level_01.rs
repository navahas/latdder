use crate::toast::api::*;
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

pub fn target() {
    let _ = Toaster::<Unplugged>::new().plug_in().toast();
}
