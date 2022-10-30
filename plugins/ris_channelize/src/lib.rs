mod channel_tracker;

use channel_tracker::ChannelTracker;
use nih_plug::prelude::*;
use rismidi::{param::OptionalMidiChannelParam, HasChannel, MidiChannel};
use std::sync::Arc;

const MIDI_CHANNEL_FROM_NIH_PLUG: &str = "MIDI channels from nih_plug must be in range 0..=15";

struct RisChannelize {
    params: Arc<RisChannelizeParams>,
    channel_tracker: ChannelTracker,
}

#[derive(Params)]
struct RisChannelizeParams {
    #[id = "target_channel"]
    pub target_channel: OptionalMidiChannelParam,
}

impl Default for RisChannelize {
    fn default() -> Self {
        Self {
            params: Arc::new(RisChannelizeParams::default()),
            channel_tracker: ChannelTracker::new(),
        }
    }
}

impl Default for RisChannelizeParams {
    fn default() -> Self {
        Self {
            target_channel: OptionalMidiChannelParam::new("Target Channel", None)
                .with_none_selected_description("No Change"),
        }
    }
}

impl RisChannelize {
    fn transform_event(
        &mut self,
        in_event: NoteEvent,
        target_chn: Option<MidiChannel>,
    ) -> NoteEvent {
        match in_event {
            NoteEvent::NoteOff {
                timing: _,
                voice_id: _,
                channel,
                note,
                velocity: _,
            } => {
                // If this is a "Note Off" event, set the channel to the one that the corresponding
                // "Note On" event was sent to, in order to avoid hanging notes.

                let out_channel = self.channel_tracker.get(
                    note,
                    MidiChannel::try_from_0_based(channel.into())
                        .expect(MIDI_CHANNEL_FROM_NIH_PLUG),
                );

                in_event.with_channel(out_channel)
            }
            _ => {
                // Move the event to the target channel, if any was set.
                let out_event = match target_chn {
                    Some(channel) => in_event.with_channel(channel),
                    None => in_event,
                };

                // If this is a "Note On" event, store the output channel that we are sending
                // this to, so we can send the corresponding "Note Off" event there too.
                if let NoteEvent::NoteOn {
                    timing: _,
                    voice_id: _,
                    channel,
                    note,
                    velocity: _,
                } = in_event
                {
                    let in_channel = MidiChannel::try_from_0_based(channel.into())
                        .expect(MIDI_CHANNEL_FROM_NIH_PLUG);
                    let out_channel = target_chn.unwrap_or(in_channel);

                    self.channel_tracker.set(note, in_channel, out_channel);
                }

                out_event
            }
        }
    }
}

impl Plugin for RisChannelize {
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const VENDOR: &'static str = "Simon Leiner";
    const EMAIL: &'static str = "rismidi@leiner.me";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const DEFAULT_INPUT_CHANNELS: u32 = 0;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 0;

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn accepts_bus_config(&self, config: &BusConfig) -> bool {
        let no_aux_busses = AuxiliaryIOConfig {
            num_busses: 0,
            num_channels: 0,
        };

        config.num_input_channels == 0
            && config.num_output_channels == 0
            && config.aux_input_busses == no_aux_busses
            && config.aux_output_busses == no_aux_busses
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(in_event) = context.next_event() {
            let target_chn = self.params.target_channel.plain_value();
            let out_event = self.transform_event(in_event, target_chn);
            context.send_event(out_event);
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for RisChannelize {
    const CLAP_ID: &'static str = "me.leiner.ris.channelize";
    const CLAP_DESCRIPTION: Option<&'static str> = Some(env!("CARGO_PKG_DESCRIPTION"));
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] =
        &[ClapFeature::Custom("MIDI"), ClapFeature::Utility];
}

impl Vst3Plugin for RisChannelize {
    const VST3_CLASS_ID: [u8; 16] = *b"risChannelize...";
    const VST3_CATEGORIES: &'static str = "Fx|Tools";
}

nih_export_clap!(RisChannelize);
nih_export_vst3!(RisChannelize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_events_are_transformed() {
        let mut processor = RisChannelize::default();

        {
            let in_event = NoteEvent::NoteOn {
                timing: 123,
                voice_id: None,
                channel: 3,
                note: 54,
                velocity: 0.6,
            };

            let out_event = processor.transform_event(in_event, Some(MidiChannel::Channel12));
            assert_eq!(out_event, in_event.with_channel(MidiChannel::Channel12));
        }
        {
            let in_event = NoteEvent::NoteOff {
                timing: 123,
                voice_id: None,
                channel: 3,
                note: 54,
                velocity: 0.6,
            };

            let out_event = processor.transform_event(in_event, Some(MidiChannel::Channel12));
            assert_eq!(out_event, in_event.with_channel(MidiChannel::Channel12));
        }
    }

    #[test]
    fn note_off_follows_note_on_channel() {
        let mut processor = RisChannelize::default();

        {
            let in_event = NoteEvent::NoteOn {
                timing: 123,
                voice_id: None,
                channel: 3,
                note: 54,
                velocity: 0.6,
            };

            let out_event = processor.transform_event(in_event, Some(MidiChannel::Channel12));
            assert_eq!(out_event, in_event.with_channel(MidiChannel::Channel12));
        }
        {
            let in_event = NoteEvent::NoteOff {
                timing: 123,
                voice_id: None,
                channel: 3,
                note: 54,
                velocity: 0.6,
            };

            let out_event = processor.transform_event(in_event, Some(MidiChannel::Channel9));
            assert_eq!(out_event, in_event.with_channel(MidiChannel::Channel12));
        }
    }

    #[test]
    fn no_transform_if_no_channel_selected() {
        let mut processor = RisChannelize::default();

        {
            let in_event = NoteEvent::NoteOn {
                timing: 123,
                voice_id: None,
                channel: 3,
                note: 54,
                velocity: 0.6,
            };

            let out_event = processor.transform_event(in_event, None);
            assert_eq!(out_event, in_event.with_channel(MidiChannel::Channel4));
        }
        {
            let in_event = NoteEvent::NoteOff {
                timing: 123,
                voice_id: None,
                channel: 3,
                note: 54,
                velocity: 0.6,
            };

            let out_event = processor.transform_event(in_event, None);
            assert_eq!(out_event, in_event.with_channel(MidiChannel::Channel4));
        }
    }
}
