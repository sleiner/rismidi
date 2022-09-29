// #![warn(missing_docs)] FIXME

mod constants;
mod error;
pub mod param;
mod switch_channel;

pub use constants::*;
pub use error::RismidiError;
pub use param::MidiChannel;
pub use switch_channel::SwitchChannel;
