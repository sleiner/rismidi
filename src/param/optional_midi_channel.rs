use nih_plug::prelude::*;

use crate::RismidiError;

use super::MidiChannel;

/// A parameter representing either a MIDI channel or no selection at all.
/// It is intended for use with [`nih_plug::param::EnumParam`].
///
/// # Examples
///
/// ```
/// use nih_plug::prelude::*;
/// use rismidi::param::OptionalMidiChannel;
///
/// #[derive(Params)]
/// struct MyPluginParams {
///     pub channel_selector: EnumParam<OptionalMidiChannel>,
/// }
/// ```
#[derive(Enum, Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum OptionalMidiChannel {
    #[id = "none"]
    #[name = "No Channel"]
    None,

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

impl OptionalMidiChannel {
    /// If a channel is selected, get its index as a 0-based [`u8`].
    ///
    /// # Examples
    ///
    /// If a channel is selected, you will get a [`u8`]:
    /// ```
    /// use rismidi::param::OptionalMidiChannel;
    ///
    /// let channel = OptionalMidiChannel::Channel1;
    /// assert_eq!(channel.try_to_0_based(), Ok(0));
    /// ```
    ///
    /// Else, you will get a [`RismidiError`];
    /// ```
    /// use rismidi::{param::OptionalMidiChannel, RismidiError};
    ///
    /// let channel = OptionalMidiChannel::None;
    /// assert_eq!(channel.try_to_0_based(), Err(RismidiError::NoChannelSelected));
    /// ```
    pub fn try_to_0_based(&self) -> Result<u8, RismidiError> {
        self.try_to_1_based().map(|c| c - 1)
    }

    /// If a channel is selected, get its index as a 1-based [`u8`].
    ///
    /// # Examples
    ///
    /// If a channel is selected, you will get a [`u8`]:
    /// ```
    /// use rismidi::param::OptionalMidiChannel;
    ///
    /// let channel = OptionalMidiChannel::Channel1;
    /// assert_eq!(channel.try_to_1_based(), Ok(1));
    /// ```
    ///
    /// Else, you will get a [`RismidiError`];
    /// ```
    /// use rismidi::{param::OptionalMidiChannel, RismidiError};
    ///
    /// let channel = OptionalMidiChannel::None;
    /// assert_eq!(channel.try_to_1_based(), Err(RismidiError::NoChannelSelected));
    /// ```
    pub fn try_to_1_based(&self) -> Result<u8, RismidiError> {
        match self {
            OptionalMidiChannel::None => Err(RismidiError::NoChannelSelected),
            _ => {
                // ASSUMPTIONS:
                //   - The index is no greater than u8::MAX
                //       => see test num_variants_is_17
                //   - The index _is_ the 1-based channel number
                //       => see test index_is_1_based_channel_number
                Ok(self.to_index().try_into().unwrap())
            }
        }
    }

    /// Turns a 0-based channel number into a [`OptionalMidiChannel`].
    ///
    /// # Examples
    ///
    /// For a valid channel index, you will get a [`OptionalMidiChannel`] instance:
    ///
    /// ```
    /// use rismidi::param::OptionalMidiChannel;
    ///
    /// let channel = OptionalMidiChannel::try_from_0_based(0).unwrap();
    /// assert_eq!(channel, OptionalMidiChannel::Channel1);
    /// ```
    ///
    /// For invalid indices, [`crate::RismidiError`] will be returned:
    ///
    /// ```
    /// use rismidi::param::OptionalMidiChannel;
    ///
    /// let channel = OptionalMidiChannel::try_from_0_based(16);
    /// assert!(channel.is_err());
    /// ```
    pub fn try_from_0_based(channel: usize) -> Result<OptionalMidiChannel, RismidiError> {
        Self::try_from_1_based(channel.wrapping_add(1)).map_err(|_| RismidiError::UIntOutOfBounds {
            found: channel,
            min: 0,
            max: Self::variants().len() - 2,
        })
    }

    /// Turns a 1-based channel number into a [`OptionalMidiChannel`].
    ///
    /// # Examples
    ///
    /// For a valid channel index, you will get a [`OptionalMidiChannel`] instance:
    ///
    /// ```
    /// use rismidi::param::OptionalMidiChannel;
    ///
    /// let channel = OptionalMidiChannel::try_from_1_based(1).unwrap();
    /// assert_eq!(channel, OptionalMidiChannel::Channel1);
    /// ```
    ///
    /// For invalid indices, [`crate::RismidiError`] will be returned:
    ///
    /// ```
    /// use rismidi::param::OptionalMidiChannel;
    ///
    /// let channel = OptionalMidiChannel::try_from_1_based(0);
    /// assert!(channel.is_err());
    /// ```
    pub fn try_from_1_based(channel: usize) -> Result<OptionalMidiChannel, RismidiError> {
        let min_index: usize = 1;
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
}

impl TryFrom<OptionalMidiChannel> for MidiChannel {
    type Error = RismidiError;

    fn try_from(value: OptionalMidiChannel) -> Result<Self, Self::Error> {
        MidiChannel::try_from_1_based(value.try_to_1_based()? as usize)
    }
}

impl From<MidiChannel> for OptionalMidiChannel {
    fn from(channel: MidiChannel) -> Self {
        // Since assume that MidiChannel::to_1_based() returns a valid index for
        // OptionalMidiChannel::try_from_1_based(), we can safely unwrap here.
        OptionalMidiChannel::try_from_1_based(channel.to_1_based() as usize).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn midi_channel_from_none() {
        let c = MidiChannel::try_from(OptionalMidiChannel::None);
        assert!(matches!(c, Err(RismidiError::NoChannelSelected)));
    }

    #[test]
    fn midi_channel_from_some() {
        for i in 1..=16 {
            let opt_channel = OptionalMidiChannel::try_from_1_based(i).unwrap();
            assert_eq!(opt_channel.try_to_1_based(), Ok(i as u8));

            let channel = MidiChannel::try_from(opt_channel).unwrap();
            assert_eq!(channel.to_1_based(), opt_channel.try_to_1_based().unwrap());
        }
    }

    #[test]
    fn num_variants_is_17() {
        assert_eq!(OptionalMidiChannel::variants().len(), 17);
    }

    #[test]
    fn index_is_1_based_channel_number() {
        for i in 1..=16 {
            let channel = OptionalMidiChannel::from_index(i);
            assert_eq!(channel.try_to_1_based(), Ok(i as u8));
        }
    }
}
