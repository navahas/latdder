//! # Init — Level 1: Getting Started
//! GOAL: Introduction to the Latdder project and basic typestate usage.
//!
//! This is your "Hello World" for understanding how the project works.
//! All examples here compile successfully to demonstrate the basic API.
//!
//! ## ✅ Basic Usage
//! ```rust
//! use latdder::init::api::*;
//! let _ = Latdder::<Ready>::start();
//! ```
//!
//! ## ✅ Understanding the Pattern
//! ```rust
//! use latdder::init::api::*;
//! // The Latdder type uses phantom types to track state
//! // Ready is a zero-sized type that exists only at compile time
//! let latdder = Latdder::<Ready>::start();
//! // This demonstrates type-driven design in action
//! ```
