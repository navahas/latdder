// Level 3 – Timer Setting with Trait Capability
//
// Add a `.set_timer()` method that is **only available once the toaster is plugged in**.
// This uses traits (`CanSetTimer`) to gate functionality by type.
//
// - `.set_timer()` should not compile if toaster is unplugged
// - `.toast()` still requires bread to be inserted
//
// Teaches: trait-based capability gating + type-state coordination
// let _ = Toaster::<Unplugged, NoBread>::new()
//     .plug_in()
//     .insert_bread()
//     .set_timer(10)
//     .toast();
