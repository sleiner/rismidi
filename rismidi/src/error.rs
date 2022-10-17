use displaydoc::Display;
use thiserror::Error;

/// Error type of this [`crate`].
#[derive(Display, Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum RismidiError {
    /// expected an integer between {min} and {max}, but found {found}
    IntOutOfBounds { found: i32, min: i32, max: i32 },

    /// expected an unsigned integer between {min} and {max}, but found {found}
    UIntOutOfBounds {
        found: usize,
        min: usize,
        max: usize,
    },

    /// message does not have a channel
    MsgHasNoChannel,

    /// the plugin host has returned an unknown value for user input
    UnknownInput,
}
