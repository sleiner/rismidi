use crate::{MidiChannel, RismidiError};
use nih_plug::midi::NoteEvent;

/// This trait represents the fact that a type can be assigned a MIDI channel.
pub trait HasChannel {
    /// Gets the current MIDI channel, if any.
    fn get_channel(&self) -> Result<MidiChannel, RismidiError>;

    /// Sets the channel for the current object, if it has any.
    fn set_channel(&mut self, new_channel: MidiChannel) -> Result<(), RismidiError>;

    /// Returns the same event, but if it has a MIDI channel, it will be overwritten.
    fn with_channel(self, channel: MidiChannel) -> Self;
}

impl<S> HasChannel for NoteEvent<S> {
    fn get_channel(&self) -> Result<MidiChannel, RismidiError> {
        let channel_index = self.channel().ok_or(RismidiError::MsgHasNoChannel);
        MidiChannel::try_from_0_based(channel_index? as usize)
    }

    fn with_channel(mut self, channel: MidiChannel) -> Self {
        // If the message does not have a channel, that is fine - we will just pass it along.
        let _ = self.set_channel(channel);

        self
    }

    fn set_channel(&mut self, new_channel: MidiChannel) -> Result<(), RismidiError> {
        let channel_index = new_channel.to_0_based();

        match self {
            NoteEvent::NoteOn {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                velocity: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::NoteOff {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                velocity: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::Choke {
                timing: _,
                voice_id: _,
                channel,
                note: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::VoiceTerminated {
                timing: _,
                voice_id: _,
                channel,
                note: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::PolyModulation {
                timing: _,
                voice_id: _,
                poly_modulation_id: _,
                normalized_offset: _,
            } => Err(RismidiError::MsgHasNoChannel),
            NoteEvent::MonoAutomation {
                timing: _,
                poly_modulation_id: _,
                normalized_value: _,
            } => Err(RismidiError::MsgHasNoChannel),
            NoteEvent::PolyPressure {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                pressure: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::PolyVolume {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                gain: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::PolyPan {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                pan: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::PolyTuning {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                tuning: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::PolyVibrato {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                vibrato: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::PolyExpression {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                expression: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::PolyBrightness {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                brightness: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::MidiChannelPressure {
                timing: _,
                channel,
                pressure: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::MidiPitchBend {
                timing: _,
                channel,
                value: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::MidiCC {
                timing: _,
                channel,
                cc: _,
                value: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            NoteEvent::MidiProgramChange {
                timing: _,
                channel,
                program: _,
            } => {
                *channel = channel_index;
                Ok(())
            }
            _ => Err(RismidiError::MsgHasNoChannel),
        }
    }
}
