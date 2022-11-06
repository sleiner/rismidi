#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

mod error;
mod has_channel;
mod midi;
pub mod param;

pub use error::RismidiError;
pub use has_channel::HasChannel;
pub use midi::{constants::*, MidiChannel};
pub use param::OptionalMidiChannelParam;
