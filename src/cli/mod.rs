//! This module contains the command line interface (CLI).
//! 
//! The concept of the CLI can be described as follows:
//! 
//! For all things related to the terminal, we can seen them as a bunch
//! of [`Tty`] instances. 
//! 
//! A `Tty` can be seen as a device which can:
//! - Be read from
//! - Be written to
//! 
//! As you can imagine, just like a pipe, we can connect multiple [`Tty`] 
//! instances together to form a pipeline. That's where the `WrapperTty`
//! trait comes in. It allows us to wrap a `Tty` and provide additional
//! functionality, or do some pre-processing before passing the data to
//! the inner [`Tty`].
//! 
//! Sometimes, we may need to modify the config **inside** a [`WrapperTty`].
//! That's where the [`InnerTty`] trait comes in. It allows us to access the
//! inner [`Tty`] of a [`WrapperTty`] (by ref or mut, ofcourse).
//! 
//! Then, based on this concept, we can build a lot of useful tools, such as
//! a shell, a recorder, a tee, etc.
//!  

pub mod tty;
pub mod serial;
pub mod shell;
pub mod ssh;
pub mod asciicast;
pub mod recorder;
pub mod tee;
pub mod deansi;