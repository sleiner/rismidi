use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum RismidiError {
    #[error("expected an integer between {min} and {max}, but found {found}")]
    IntOutOfBounds { found: i32, min: i32, max: i32 },

    #[error("expected an unsigned integer between {min} and {max}, but found {found}")]
    UIntOutOfBounds {
        found: usize,
        min: usize,
        max: usize,
    },

    #[error("message does not have a channel")]
    MsgHasNoChannel,

    #[error("the plugin host has returned an unknown value for user input")]
    UnknownInput,
}
