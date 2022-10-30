use nih_plug::prelude::*;
use rismidi::{HasChannel, MidiChannel, OptionalMidiChannelParam};
use std::sync::Arc;

struct RisChannelFilter {
    params: Arc<RisChannelFilterParams>,
}

#[derive(Params)]
struct RisChannelFilterParams {
    #[id = "target_channel"]
    pub target_channel: OptionalMidiChannelParam,
}

impl Default for RisChannelFilter {
    fn default() -> Self {
        Self {
            params: Arc::new(RisChannelFilterParams::default()),
        }
    }
}

impl Default for RisChannelFilterParams {
    fn default() -> Self {
        Self {
            target_channel: OptionalMidiChannelParam::new("Target Channel", None)
                .with_none_selected_description("All"),
        }
    }
}

impl RisChannelFilter {
    fn transform_event(
        &mut self,
        in_event: NoteEvent,
        filter_chn: Option<MidiChannel>,
    ) -> Option<NoteEvent> {
        match filter_chn {
            None => Some(in_event),
            Some(filter_channel) => match in_event.get_channel() {
                Err(_) => Some(in_event),
                Ok(in_channel) => {
                    if in_channel == filter_channel {
                        Some(in_event)
                    } else {
                        None
                    }
                }
            },
        }
    }
}

impl Plugin for RisChannelFilter {
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
            if let Some(out_event) = self.transform_event(in_event, target_chn) {
                context.send_event(out_event);
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for RisChannelFilter {
    const CLAP_ID: &'static str = "me.leiner.ris.channel_filter";
    const CLAP_DESCRIPTION: Option<&'static str> = Some(env!("CARGO_PKG_DESCRIPTION"));
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] =
        &[ClapFeature::Custom("MIDI"), ClapFeature::Utility];
}

impl Vst3Plugin for RisChannelFilter {
    const VST3_CLASS_ID: [u8; 16] = *b"risChannelFilter";
    const VST3_CATEGORIES: &'static str = "Fx|Tools";
}

nih_export_clap!(RisChannelFilter);
nih_export_vst3!(RisChannelFilter);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_note_event_on_filter_channel() {
        let mut processor = RisChannelFilter::default();

        let in_event = NoteEvent::NoteOn {
            timing: 123,
            voice_id: None,
            channel: 11,
            note: 54,
            velocity: 0.6,
        };

        let out_event = processor.transform_event(in_event, Some(MidiChannel::Channel12));
        assert_eq!(out_event, Some(in_event));
    }

    #[test]
    fn transform_note_event_on_ignored_channel() {
        let mut processor = RisChannelFilter::default();

        let in_event = NoteEvent::NoteOn {
            timing: 123,
            voice_id: None,
            channel: 13,
            note: 54,
            velocity: 0.6,
        };

        let out_event = processor.transform_event(in_event, Some(MidiChannel::Channel12));
        assert_eq!(out_event, None);
    }

    #[test]
    fn transform_note_event_with_no_channel_selected() {
        let mut processor = RisChannelFilter::default();

        let in_event = NoteEvent::NoteOn {
            timing: 123,
            voice_id: None,
            channel: 13,
            note: 54,
            velocity: 0.6,
        };

        let out_event = processor.transform_event(in_event, None);
        assert_eq!(out_event, Some(in_event));
    }

    #[test]
    fn transform_event_without_channel() {
        let mut processor = RisChannelFilter::default();

        for filter_chn in [Some(MidiChannel::Channel12), None] {
            let in_event = NoteEvent::PolyModulation {
                timing: 0,
                voice_id: 1,
                poly_modulation_id: 2,
                normalized_offset: 3.0,
            };

            let out_event = processor.transform_event(in_event, filter_chn);
            assert_eq!(out_event, Some(in_event));
        }
    }
}
