use crate::{MidiChannel, RismidiError};
use nih_plug::midi::NoteEvent;

pub trait HasChannel {
    fn get_channel(&self) -> Result<MidiChannel, RismidiError>;

    fn set_channel(&mut self, new_channel: MidiChannel) -> Result<(), RismidiError>;

    fn with_channel(self, channel: MidiChannel) -> Self;
}

impl HasChannel for NoteEvent {
    fn get_channel(&self) -> Result<MidiChannel, RismidiError> {
        let channel_index = match self {
            NoteEvent::NoteOn {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                velocity: _,
            } => Ok(channel),
            NoteEvent::NoteOff {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                velocity: _,
            } => Ok(channel),
            NoteEvent::Choke {
                timing: _,
                voice_id: _,
                channel,
                note: _,
            } => Ok(channel),
            NoteEvent::VoiceTerminated {
                timing: _,
                voice_id: _,
                channel,
                note: _,
            } => Ok(channel),
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
            } => Ok(channel),
            NoteEvent::PolyVolume {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                gain: _,
            } => Ok(channel),
            NoteEvent::PolyPan {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                pan: _,
            } => Ok(channel),
            NoteEvent::PolyTuning {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                tuning: _,
            } => Ok(channel),
            NoteEvent::PolyVibrato {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                vibrato: _,
            } => Ok(channel),
            NoteEvent::PolyExpression {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                expression: _,
            } => Ok(channel),
            NoteEvent::PolyBrightness {
                timing: _,
                voice_id: _,
                channel,
                note: _,
                brightness: _,
            } => Ok(channel),
            NoteEvent::MidiChannelPressure {
                timing: _,
                channel,
                pressure: _,
            } => Ok(channel),
            NoteEvent::MidiPitchBend {
                timing: _,
                channel,
                value: _,
            } => Ok(channel),
            NoteEvent::MidiCC {
                timing: _,
                channel,
                cc: _,
                value: _,
            } => Ok(channel),
            NoteEvent::MidiProgramChange {
                timing: _,
                channel,
                program: _,
            } => Ok(channel),
            _ => Err(RismidiError::MsgHasNoChannel),
        };

        MidiChannel::try_from_0_based(*channel_index? as usize)
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
