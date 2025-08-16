#![doc = include_str!("../README.md")]

//! # Complete Learning Guide
//! 
//! **New to Latdder?** Start with the **[Main Guide](https://github.com/navahas/latdder/blob/master/doc/MAIN_GUIDE.md)** for a complete overview.
//!
//! ## Quick Navigation
//! 
//! - **[Core Concepts](https://github.com/navahas/latdder/blob/master/doc/CONCEPTS.md)**: Essential theory (phantom types, typestate pattern, etc.)
//! - **[Theme Guides](https://github.com/navahas/latdder/blob/master/doc/THEMES.md)**: Step-by-step implementation guides
//! - **[Resources](https://github.com/navahas/latdder/blob/master/doc/RESOURCES.md)**: Additional learning materials
//!
//! ## Learning Themes
//! 
//! ### Init Theme (Start Here!)
//! - **[Level 1](crate::init::levels::level_01)**: Introduction to typestate patterns
//!
//! ### Toast Theme (Main Curriculum)
//! - **[Level 1](crate::toast::levels::level_01)**: Basic typestate - prevent toasting when unplugged
//! - **[Level 2](crate::toast::levels::level_02)**: Multi-state validation  
//! - **[Level 3](crate::toast::levels::level_03)**: Enhanced state tracking
//! - **[Level 4](crate::toast::levels::level_04)**: Complex transitions and error handling
//! - **[Level 5](crate::toast::levels::level_05)**: Advanced patterns and optimization
//!
//! ## Additional Resources
//! 
//! Explore additional learning materials in the [resources](crate::resources) module.

//pub use self::resources::*;
pub mod resources {
    //! # Additional Resources
    //!
    //! ## External Learning Materials
    //! - **[Will Crichton - Rust API Type Patterns](https://willcrichton.net/rust-api-type-patterns/)**
    //! - **[Jon Gjengset YouTube Channel](https://www.youtube.com/c/JonGjengset)**
    //!
    //! ## Deep Dive Content
    //! - **[Type System Trivia](crate::resources::trivia_rust_types)**: Fascinating insights from Jon Gjengset
    //!
    //! ## Complete Documentation
    //! For comprehensive guides, visit our [documentation directory](https://github.com/navahas/latdder/tree/master/doc).
    
    pub mod trivia_rust_types {
        #![allow(rustdoc::bare_urls)]
        #![doc = include_str!("../doc/TRIVIA_RUST_TYPES.md")]
    }
}

pub mod init {
    #[cfg(not(doc))]
    pub mod api;
    pub mod levels {
        pub mod level_01;
    }
}

pub mod toast {
    //! # Levels
    //! - **Toast:** [Level 1](crate::toast::levels::level_01),
    //!   [2](crate::toast::levels::level_02),
    //!   [3](crate::toast::levels::level_03),
    //!   [4](crate::toast::levels::level_04),
    //!   [5](crate::toast::levels::level_05)

    #[doc(hidden)]
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
