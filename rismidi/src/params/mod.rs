//! More plugin parameters in addition to [`nih_plug::params`].

mod optional_int;
mod optional_midi_channel;
mod percentage;
mod simple_time;

pub use optional_midi_channel::OptionalMidiChannelParam;
pub use percentage::PercentageParamBuilder;
pub use simple_time::{SimpleTime, SimpleTimeParams};
