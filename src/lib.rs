// #![warn(missing_docs)] FIXME

mod constants;
mod error;
mod has_channel;
pub mod param;

pub use constants::*;
pub use error::RismidiError;
pub use has_channel::HasChannel;
pub use param::{MidiChannel, OptionalMidiChannelParam};
