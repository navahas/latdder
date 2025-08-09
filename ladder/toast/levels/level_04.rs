// Level 4 – Ejecting Toast
//
// In this level, you can only call `.eject()` **after** `.toast()`.
// This introduces a new state: `Toasted`, which must be present before `eject()` is available.
//
// Teaches: multiple chained state transitions, enforcing strict action order
// fn valid_eject_sequence() {
//     let _ = Toaster::<Unplugged, NoBread>::new()
//         .plug_in()
//         .insert_bread()
//         .set_timer(3)
//         .toast()
//         .eject();
// }

