#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::restriction)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![doc = include_str!("../README.md")]

mod error;
mod has_channel;
mod midi;
pub mod params;
mod time;

pub use error::RismidiError;
pub use has_channel::HasChannel;
pub use midi::{constants::*, MidiChannel};
pub use params::OptionalMidiChannelParam;
pub use time::{NoteDuration, TimeDuration, Tuplet};
