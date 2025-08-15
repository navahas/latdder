pub mod init {
    pub mod api;
    pub mod levels {
        pub mod level_01;
    }
}

pub mod toast {
    pub mod api;
    pub mod levels {
        // Level 1 — visible in docs, gated in code
        #[cfg_attr(
            not(doc),
            cfg(all(
                feature = "toast_level_01",
                not(feature = "toast_level_02"),
                not(feature = "toast_level_03"),
                not(feature = "toast_level_04"),
                not(feature = "toast_level_05"),
            ))
        )]
        pub mod level_01;

        // Level 2
        #[cfg_attr(
            not(doc),
            cfg(all(
                feature = "toast_level_02",
                not(feature = "toast_level_03"),
                not(feature = "toast_level_04"),
                not(feature = "toast_level_05"),
            ))
        )]
        pub mod level_02;

        // Level 3
        #[cfg_attr(
            not(doc),
            cfg(all(
                feature = "toast_level_03",
                not(feature = "toast_level_04"),
                not(feature = "toast_level_05"),
            ))
        )]
        pub mod level_03;

        // Level 4
        #[cfg_attr(
            not(doc),
            cfg(all(feature = "toast_level_04", not(feature = "toast_level_05")))
        )]
        pub mod level_04;

        // Level 5
        #[cfg_attr(not(doc), cfg(feature = "toast_level_05"))]
        pub mod level_05;
    }
}
