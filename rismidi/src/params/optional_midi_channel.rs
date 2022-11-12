use crate::{MidiChannel, RismidiError};
use nih_plug::prelude::*;
use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

/// A plugin parameter modelling either the selection of a MIDI channel or the explicit selection of no channel.
pub struct OptionalMidiChannelParam {
    /// As this is a parameter with a finite choice of (mostly) sequential options, we delegate most of the heavy
    /// lifting to this.
    inner: IntParam,

    /// We store the default here in addition to `inner.default` to avoid runtime conversions in
    /// [`Param::default_plain_value`].
    default: Option<MidiChannel>,

    /// The parameter description visible in the plugin host if no channel is selected.
    description: &'static str,
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
            description: "",
        };

        instance.with_none_selected_description(Self::DEFAULT_NO_CHANNEL_DESCRIPTION)
    }

    /// The field's current plain value, after monophonic modulation has been applied. Equivalent
    /// to calling `param.modulated_plain_value()`.
    #[inline]
    pub fn value(&self) -> Option<MidiChannel> {
        self.modulated_plain_value()
    }

    /// Sets the description of the "no channel selected" position. Usually, this will be shown to
    /// the user by the plugin host.
    pub fn with_none_selected_description(mut self, description: &'static str) -> Self {
        self.description = description;

        self.with_updated_callbacks()
    }

    fn with_updated_callbacks(mut self) -> Self {
        self.inner = self
            .inner
            .with_value_to_string(Arc::new(|index| {
                let selection = Self::try_selection_from_inner(index).unwrap_or_default();
                Self::selection_to_string(selection, self.description)
            }))
            .with_string_to_value(Arc::new(|string| {
                let selection = Self::try_selection_from_string(string, self.description).ok()?;
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
        let string = Self::selection_to_string(self.modulated_plain_value(), self.description);
        write!(f, "{}", string)
    }
}

impl Debug for OptionalMidiChannelParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptMidiChannelParam")
            .field("channel", &self.modulated_plain_value())
            .field("default", &self.default)
            .field("no_channel_msg", &self.description)
            .finish()
    }
}

impl Param for OptionalMidiChannelParam {
    type Plain = Option<MidiChannel>;

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn unit(&self) -> &'static str {
        self.inner.unit()
    }

    fn poly_modulation_id(&self) -> Option<u32> {
        self.inner.poly_modulation_id()
    }

    fn modulated_plain_value(&self) -> Self::Plain {
        Self::try_selection_from_inner(self.inner.modulated_plain_value()).unwrap_or(self.default)
    }

    fn modulated_normalized_value(&self) -> f32 {
        self.inner.modulated_normalized_value()
    }

    fn unmodulated_plain_value(&self) -> Self::Plain {
        Self::try_selection_from_inner(self.inner.unmodulated_plain_value()).unwrap_or(self.default)
    }

    fn unmodulated_normalized_value(&self) -> f32 {
        self.inner.unmodulated_normalized_value()
    }

    fn default_plain_value(&self) -> Self::Plain {
        self.default
    }

    fn step_count(&self) -> Option<usize> {
        self.inner.step_count()
    }

    fn previous_step(&self, from: Self::Plain, finer: bool) -> Self::Plain {
        Self::try_selection_from_inner(
            self.inner
                .previous_step(Self::selection_to_inner(from), finer),
        )
        .unwrap_or(self.default)
    }

    fn next_step(&self, from: Self::Plain, finer: bool) -> Self::Plain {
        Self::try_selection_from_inner(self.inner.next_step(Self::selection_to_inner(from), finer))
            .unwrap_or(self.default)
    }

    fn normalized_value_to_string(&self, _normalized: f32, _include_unit: bool) -> String {
        // This function is usually never called - nih_plug calls IntParam's implementation of this function.
        nih_debug_assert_failure!(
            "Did not expect a call of OptionalMidiChannelParam::normalized_value_to_string()"
        );

        String::from("")
    }

    fn string_to_normalized_value(&self, _string: &str) -> Option<f32> {
        // This function is usually never called - nih_plug calls IntParam's implementation of this function.
        nih_debug_assert_failure!(
            "Did not expect a call of OptionalMidiChannelParam::string_to_modulated_normalized_value()"
        );

        None
    }

    fn preview_normalized(&self, plain: Self::Plain) -> f32 {
        Self::RANGE.normalize(Self::selection_to_inner(plain))
    }

    fn preview_plain(&self, normalized: f32) -> Self::Plain {
        Self::try_selection_from_inner(Self::RANGE.unnormalize(normalized)).unwrap_or(self.default)
    }

    fn flags(&self) -> ParamFlags {
        self.inner.flags()
    }

    fn as_ptr(&self) -> ParamPtr {
        self.inner.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn current_description(param: impl Param) -> String {
        let p = param.as_ptr();
        unsafe { p.normalized_value_to_string(p.modulated_normalized_value(), true) }
    }

    #[test]
    fn description_when_none_selected() {
        let no_channel_description = "this is a test string";
        let param = OptionalMidiChannelParam::new("test", None)
            .with_none_selected_description(no_channel_description);

        // param is still in the default position: no channel selected
        assert_eq!(current_description(param), no_channel_description);
    }

    #[test]
    fn description_when_channel_selected() {
        for i in 1..=16 {
            let channel = MidiChannel::try_from_1_based(i).unwrap();
            let param = OptionalMidiChannelParam::new("test", Some(channel));

            assert_eq!(current_description(param), format!("{i}"));
        }
    }
}
