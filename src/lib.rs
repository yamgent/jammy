// SPDX-License-Identifier: MIT OR Apache-2.0

//! jammy is a collection of libraries that allows you to quickly prototype
//! games in bevy. The typical use case is for game jams (hence the name "Jammy").
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod splash_screen {
    //! A simple splash screen with a loading bar
    pub use jammy_splash_screen::*;
}
