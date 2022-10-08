use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum RismidiError {
    #[error("expected an integer between {min} and {max}, but found {found}")]
    UIntOutOfBounds {
        found: usize,
        min: usize,
        max: usize,
    },

    #[error("no channel was selected")]
    NoChannelSelected,

    #[error("message does not have a channel")]
    MsgHasNoChannel,
}
