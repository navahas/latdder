#[cfg(all(
    feature = "toast_level_01",
    not(feature = "toast_level_02"),
    not(feature = "toast_level_03"),
    not(feature = "toast_level_04"),
    not(feature = "toast_level_05"),
))]
#[path = "toast/level_01.rs"]
mod level_01;

#[cfg(all(
    feature = "toast_level_02",
    not(feature = "toast_level_03"),
    not(feature = "toast_level_04"),
    not(feature = "toast_level_05"),
))]
#[path = "toast/level_02.rs"]
mod level_02;

// (Add level_03/04/05 similarly, picking the highest)

// Optional: re-export for nicer filtering if you want
#[cfg(all(
    feature = "toast_level_01",
    not(feature = "toast_level_02"),
    not(feature = "toast_level_03"),
    not(feature = "toast_level_04"),
    not(feature = "toast_level_05"),
))]

#[cfg(all(
    feature = "toast_level_02",
    not(feature = "toast_level_03"),
    not(feature = "toast_level_04"),
    not(feature = "toast_level_05"),
))]
pub use level_02::*;
