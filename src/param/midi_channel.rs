use crate::RismidiError;
use nih_plug::prelude::*;

/// Represents a MIDI channel.
/// It can be used as a plugin parameter with [`nih_plug::param::EnumParam`].
///
/// # Examples
///
/// ```
/// use nih_plug::prelude::*;
/// use rismidi::MidiChannel;
///
/// #[derive(Params)]
/// struct MyPluginParams {
///     pub channel_selector: EnumParam<MidiChannel>,
/// }
/// ```
#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MidiChannel {
    #[id = "1"]
    #[name = "1"]
    Channel1,

    #[id = "2"]
    #[name = "2"]
    Channel2,

    #[id = "3"]
    #[name = "3"]
    Channel3,

    #[id = "4"]
    #[name = "4"]
    Channel4,

    #[id = "5"]
    #[name = "5"]
    Channel5,

    #[id = "6"]
    #[name = "6"]
    Channel6,

    #[id = "7"]
    #[name = "7"]
    Channel7,

    #[id = "8"]
    #[name = "8"]
    Channel8,

    #[id = "9"]
    #[name = "9"]
    Channel9,

    #[id = "10"]
    #[name = "10"]
    Channel10,

    #[id = "11"]
    #[name = "11"]
    Channel11,

    #[id = "12"]
    #[name = "12"]
    Channel12,

    #[id = "13"]
    #[name = "13"]
    Channel13,

    #[id = "14"]
    #[name = "14"]
    Channel14,

    #[id = "15"]
    #[name = "15"]
    Channel15,

    #[id = "16"]
    #[name = "16"]
    Channel16,
}

impl MidiChannel {
    /// Get the channel's index as a 0-based [`u8`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rismidi::MidiChannel;
    ///
    /// let channel = MidiChannel::Channel1;
    /// let channel_idx = channel.to_0_based();
    /// assert_eq!(channel_idx, 0);
    /// ```
    pub fn to_0_based(&self) -> u8 {
        self.to_index() as u8
    }

    /// Get the channel's index as a 1-based [`u8`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rismidi::MidiChannel;
    ///
    /// let channel = MidiChannel::Channel1;
    /// let channel_idx = channel.to_1_based();
    /// assert_eq!(channel_idx, 1);
    /// ```
    pub fn to_1_based(&self) -> u8 {
        self.to_0_based() + 1
    }

    /// Turns a 0-based channel number into a [`MidiChannel`].
    ///
    /// # Examples
    ///
    /// For a valid channel index, you will get a [`MidiChannel`] instance:
    ///
    /// ```
    /// use rismidi::MidiChannel;
    ///
    /// let channel = MidiChannel::try_from_0_based(0).unwrap();
    /// assert_eq!(channel, MidiChannel::Channel1);
    /// ```
    ///
    /// For invalid indices, [`crate::RismidiError`] will be returned:
    ///
    /// ```
    /// use rismidi::MidiChannel;
    ///
    /// let channel = MidiChannel::try_from_0_based(16);
    /// assert!(channel.is_err());
    /// ```
    pub fn try_from_0_based(channel: usize) -> Result<MidiChannel, RismidiError> {
        let min_index: usize = 0;
        let max_index = Self::variants().len() - 1;

        if min_index <= channel && channel <= max_index {
            Ok(Self::from_index(channel))
        } else {
            Err(RismidiError::UIntOutOfBounds {
                found: channel,
                min: min_index,
                max: max_index,
            })
        }
    }

    /// Turns a 1-based channel number into a [`MidiChannel`].
    ///
    /// # Examples
    ///
    /// For a valid channel index, you will get a [`MidiChannel`] instance:
    ///
    /// ```
    /// use rismidi::MidiChannel;
    ///
    /// let channel = MidiChannel::try_from_1_based(12).unwrap();
    /// assert_eq!(channel, MidiChannel::Channel12);
    /// ```
    ///
    /// For invalid indices, [`Option::None`] will be returned:
    ///
    /// ```
    /// use rismidi::MidiChannel;
    ///
    /// let channel = MidiChannel::try_from_1_based(0);
    /// assert!(channel.is_err());
    /// ```
    ///
    /// ```
    /// use rismidi::MidiChannel;
    ///
    /// let channel = MidiChannel::try_from_1_based(17);
    /// assert!(channel.is_err());
    /// ```
    pub fn try_from_1_based(channel: usize) -> Result<MidiChannel, RismidiError> {
        Self::try_from_0_based(channel.wrapping_sub(1)).map_err(|_| RismidiError::UIntOutOfBounds {
            found: channel,
            min: 1,
            max: Self::variants().len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_variants_matches_num_midi_channels() {
        assert_eq!(
            MidiChannel::variants().len(),
            crate::NUM_MIDI_CHANNELS as usize
        );
    }
}
