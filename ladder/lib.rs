pub mod init {
    pub mod api;
    pub mod levels {
        pub mod level_01;
    }
}

pub mod toast {
    pub mod api;
    pub mod levels {
        // If NO higher level is enabled, compile level_01
        #[cfg(all(
            feature = "toast_level_01",
            not(feature = "toast_level_02"),
            not(feature = "toast_level_03"),
            not(feature = "toast_level_04"),
            not(feature = "toast_level_05"),
        ))]
        pub mod level_01;

        // If L2 is the highest enabled, compile only level_02
        #[cfg(all(
            feature = "toast_level_02",
            not(feature = "toast_level_03"),
            not(feature = "toast_level_04"),
            not(feature = "toast_level_05"),
        ))]
        pub mod level_02;

        // Same idea for higher levels when you add them:
        #[cfg(all(
            feature = "toast_level_03",
            not(feature = "toast_level_04"),
            not(feature = "toast_level_05"),
        ))]
        pub mod level_03;

        #[cfg(all(feature = "toast_level_04", not(feature = "toast_level_05")))]
        pub mod level_04;

        #[cfg(feature = "toast_level_05")]
        pub mod level_05;
    }
}
