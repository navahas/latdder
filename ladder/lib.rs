#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod init {
    pub mod api;
    pub mod levels {
        pub mod level_01;
    }
}

pub mod toast {
    pub mod api;
    pub mod levels {
        #[cfg(all(
            feature = "toast_level_01",
            not(feature = "toast_level_02"),
            not(feature = "toast_level_03"),
            not(feature = "toast_level_04"),
            not(feature = "toast_level_05"),
        ))]
        #[cfg_attr(docsrs, doc(cfg(feature = "toast_level_01")))]
        pub mod level_01;

        #[cfg(all(
            feature = "toast_level_02",
            not(feature = "toast_level_03"),
            not(feature = "toast_level_04"),
            not(feature = "toast_level_05"),
        ))]
        #[cfg_attr(docsrs, doc(cfg(feature = "toast_level_02")))]
        pub mod level_02;

        #[cfg(all(
            feature = "toast_level_03",
            not(feature = "toast_level_04"),
            not(feature = "toast_level_05"),
        ))]
        #[cfg_attr(docsrs, doc(cfg(feature = "toast_level_03")))]
        pub mod level_03;

        #[cfg(all(feature = "toast_level_04", not(feature = "toast_level_05")))]
        #[cfg_attr(docsrs, doc(cfg(feature = "toast_level_04")))]
        pub mod level_04;

        #[cfg(feature = "toast_level_05")]
        #[cfg_attr(docsrs, doc(cfg(feature = "toast_level_05")))]
        pub mod level_05;
    }
}
