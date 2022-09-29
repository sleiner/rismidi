use crate::MidiChannel;
use nih_plug::midi::NoteEvent;

pub trait SwitchChannel {
    fn with_channel(self, channel: MidiChannel) -> Self;
}

impl SwitchChannel for NoteEvent {
    fn with_channel(self, channel: MidiChannel) -> Self {
        match self {
            Self::NoteOn {
                timing,
                voice_id,
                channel: _,
                note,
                velocity,
            } => Self::NoteOn {
                timing,
                voice_id,
                channel: channel.to_0_based(),
                note,
                velocity,
            },
            Self::NoteOff {
                timing,
                voice_id,
                channel: _,
                note,
                velocity,
            } => Self::NoteOff {
                timing,
                voice_id,
                channel: channel.to_0_based(),
                note,
                velocity,
            },
            Self::Choke {
                timing,
                voice_id,
                channel: _,
                note,
            } => Self::Choke {
                timing,
                voice_id,
                channel: channel.to_0_based(),
                note,
            },
            Self::VoiceTerminated {
                timing,
                voice_id,
                channel: _,
                note,
            } => Self::VoiceTerminated {
                timing,
                voice_id,
                channel: channel.to_0_based(),
                note,
            },
            Self::PolyPressure {
                timing,
                voice_id,
                channel: _,
                note,
                pressure,
            } => Self::PolyPressure {
                timing,
                voice_id,
                channel: channel.to_0_based(),
                note,
                pressure,
            },
            Self::PolyVolume {
                timing,
                voice_id,
                channel: _,
                note,
                gain,
            } => Self::PolyVolume {
                timing,
                voice_id,
                channel: channel.to_0_based(),
                note,
                gain,
            },
            Self::PolyPan {
                timing,
                voice_id,
                channel: _,
                note,
                pan,
            } => Self::PolyPan {
                timing,
                voice_id,
                channel: channel.to_0_based(),
                note,
                pan,
            },
            Self::PolyTuning {
                timing,
                voice_id,
                channel: _,
                note,
                tuning,
            } => Self::PolyTuning {
                timing,
                voice_id,
                channel: channel.to_0_based(),
                note,
                tuning,
            },
            Self::PolyVibrato {
                timing,
                voice_id,
                channel: _,
                note,
                vibrato,
            } => Self::PolyVibrato {
                timing,
                voice_id,
                channel: channel.to_0_based(),
                note,
                vibrato,
            },
            Self::PolyExpression {
                timing,
                voice_id,
                channel: _,
                note,
                expression,
            } => Self::PolyExpression {
                timing,
                voice_id,
                channel: channel.to_0_based(),
                note,
                expression,
            },
            Self::PolyBrightness {
                timing,
                voice_id,
                channel: _,
                note,
                brightness,
            } => Self::PolyBrightness {
                timing,
                voice_id,
                channel: channel.to_0_based(),
                note,
                brightness,
            },
            Self::MidiChannelPressure {
                timing,
                channel: _,
                pressure,
            } => Self::MidiChannelPressure {
                timing,
                channel: channel.to_0_based(),
                pressure,
            },
            Self::MidiPitchBend {
                timing,
                channel: _,
                value,
            } => Self::MidiPitchBend {
                timing,
                channel: channel.to_0_based(),
                value,
            },
            Self::MidiCC {
                timing,
                channel: _,
                cc,
                value,
            } => Self::MidiCC {
                timing,
                channel: channel.to_0_based(),
                cc,
                value,
            },
            Self::MidiProgramChange {
                timing,
                channel: _,
                program,
            } => Self::MidiProgramChange {
                timing,
                channel: channel.to_0_based(),
                program,
            },
            _ => self,
        }
    }
}
