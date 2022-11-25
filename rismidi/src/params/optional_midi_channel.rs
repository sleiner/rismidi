use crate::{MidiChannel, RismidiError};
use nih_plug::prelude::*;
use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

/// A plugin parameter modelling either the selection of a MIDI channel or the explicit selection of no channel.
#[derive(Params)]
pub struct OptionalMidiChannelParam {
    /// As this is a parameter with a finite choice of (mostly) sequential options, we delegate most of the heavy
    /// lifting to this.
    #[id = ""]
    inner: IntParam,

    /// We store the default here in addition to `inner.default` to avoid runtime conversions in
    /// [`Param::default_plain_value`].
    default: Option<MidiChannel>,

    /// The parameter description visible in the plugin host if no channel is selected.
    no_channel_description: &'static str,
}

impl OptionalMidiChannelParam {
    /// Models the transformation between integers and the normalized parameter value.
    ///
    ///   - 0 means no channel is selected.
    ///   - 1 to 16 means one specific channel.
    const RANGE: IntRange = IntRange::Linear { min: 0, max: 16 };

    /// The default description of the "no channel selected" state.
    pub const DEFAULT_NO_CHANNEL_DESCRIPTION: &str = "No Channel";

    /// Creates a new [`OptionalMidiChannelParam`].
    ///
    /// Use the other associated functions to modify the behavior of the parameter.
    pub fn new(name: impl Into<String>, default: Option<MidiChannel>) -> Self {
        let instance = Self {
            inner: IntParam::new(name, Self::selection_to_inner(default), Self::RANGE),
            default,
            no_channel_description: "",
        };

        instance.with_none_selected_description(Self::DEFAULT_NO_CHANNEL_DESCRIPTION)
    }

    /// The field's current plain value, after monophonic modulation has been applied.
    #[inline]
    pub fn value(&self) -> Option<MidiChannel> {
        Self::try_selection_from_inner(self.inner.value()).unwrap_or(self.default)
    }

    /// Returns the [`String`] representation for the current value.
    pub fn description(&self) -> String {
        let normalized = self.inner.modulated_normalized_value();
        self.inner.normalized_value_to_string(normalized, true)
    }

    /// Sets the description of the "no channel selected" position. Usually, this will be shown to
    /// the user by the plugin host.
    pub fn with_none_selected_description(mut self, description: &'static str) -> Self {
        self.no_channel_description = description;

        self.with_updated_callbacks()
    }

    fn with_updated_callbacks(mut self) -> Self {
        self.inner = self
            .inner
            .with_value_to_string(Arc::new(|index| {
                let selection = Self::try_selection_from_inner(index).unwrap_or_default();
                Self::selection_to_string(selection, self.no_channel_description)
            }))
            .with_string_to_value(Arc::new(|string| {
                let selection =
                    Self::try_selection_from_string(string, self.no_channel_description).ok()?;
                Some(Self::selection_to_inner(selection))
            }));

        self
    }

    /// Tries to convert the representation of [`Self::inner`] into an [`Option<MidiChannel>`].
    ///
    /// It will return:
    ///
    ///   - [`Ok(Some(MidiChannel)`] if `inner_val` represents the selection of a channel.
    ///   - [`Ok(None)`] if `inner_val` represents the fact that "no channel" was selected.
    ///   - [`Err`] if `inner_val` does not represent any valid selection.
    fn try_selection_from_inner(inner_val: i32) -> Result<Option<MidiChannel>, RismidiError> {
        match inner_val {
            0 => Ok(None),
            other => Ok(Some(MidiChannel::try_from_1_based(other as usize)?)),
        }
        .map_err(|_: RismidiError| RismidiError::IntOutOfBounds {
            found: inner_val,
            min: 0,
            max: 16,
        })
    }

    /// Converts an [`Option<MidiChannel>`] into the appropriate representation for [`Self::inner`].
    fn selection_to_inner(from: Option<MidiChannel>) -> i32 {
        match from {
            Some(channel) => channel.to_1_based().into(),
            None => 0,
        }
    }

    /// Converts an [`Option<MidiChannel>] into the string representation to show to the user.
    fn selection_to_string(selection: Option<MidiChannel>, no_channel_msg: &str) -> String {
        match selection {
            None => no_channel_msg.to_string(),
            Some(channel) => {
                format!("{}", channel.to_1_based())
            }
        }
    }

    /// Tries to convert the string representation of a user selection into an [`Option<MidiChannel>`].
    fn try_selection_from_string(
        description: &str,
        no_channel_msg: &str,
    ) -> Result<Option<MidiChannel>, RismidiError> {
        let string = description.trim();

        if string == no_channel_msg {
            Ok(None)
        } else {
            let index = string.parse().map_err(|_| RismidiError::UnknownInput)?;
            Self::try_selection_from_inner(index)
        }
    }
}

impl Display for OptionalMidiChannelParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = Self::selection_to_string(self.value(), self.no_channel_description);
        write!(f, "{}", string)
    }
}

impl Debug for OptionalMidiChannelParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptMidiChannelParam")
            .field("channel", &self.value())
            .field("default", &self.default)
            .field("no_channel_msg", &self.no_channel_description)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description_when_none_selected() {
        let no_channel_description = "this is a test string";
        let param = OptionalMidiChannelParam::new("test", None)
            .with_none_selected_description(no_channel_description);

        // param is still in the default position: no channel selected
        assert_eq!(param.description(), no_channel_description);
    }

    #[test]
    fn description_when_channel_selected() {
        for i in 1..=16 {
            let channel = MidiChannel::try_from_1_based(i).unwrap();
            let param = OptionalMidiChannelParam::new("test", Some(channel));

            assert_eq!(param.description(), format!("{i}"));
        }
    }
}
