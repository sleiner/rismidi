use nih_plug::prelude::*;
use rismidi::{
    params::{PercentageParamBuilder, SimpleTimeParams},
    NoteDuration,
};
use std::sync::Arc;

struct RisDelay {
    params: Arc<RisDelayParams>,
}

#[derive(Params)]
struct RisDelayParams {
    #[nested(id_prefix = "delay")]
    pub delay_time: SimpleTimeParams,

    #[id = "feedback"]
    pub feedback: FloatParam,
}

impl Default for RisDelay {
    fn default() -> Self {
        Self {
            params: Arc::new(RisDelayParams::default()),
        }
    }
}

impl Default for RisDelayParams {
    fn default() -> Self {
        Self {
            delay_time: SimpleTimeParams::new(
                "Delay",
                FloatRange::Skewed {
                    min: 1.0,
                    max: 2000.0,
                    factor: 0.35,
                },
                &[
                    (String::from("1/64"), NoteDuration::new(1, 64)),
                    (String::from("1/32T"), NoteDuration::new(1, 32).triplet()),
                    (String::from("1/64."), NoteDuration::new(1, 64).dotted(1)),
                    (String::from("1/32"), NoteDuration::new(1, 32)),
                    (String::from("1/16T"), NoteDuration::new(1, 16).triplet()),
                    (String::from("1/32."), NoteDuration::new(1, 32).dotted(1)),
                    (String::from("1/16"), NoteDuration::new(1, 16)),
                    (String::from("1/8T"), NoteDuration::new(1, 8).triplet()),
                    (String::from("1/16."), NoteDuration::new(1, 16).dotted(1)),
                    (String::from("1/8"), NoteDuration::new(1, 8)),
                    (String::from("1/4T"), NoteDuration::new(1, 4).triplet()),
                    (String::from("1/8."), NoteDuration::new(1, 8).dotted(1)),
                    (String::from("1/4"), NoteDuration::new(1, 4)),
                    (String::from("1/2T"), NoteDuration::new(1, 2).triplet()),
                    (String::from("1/4."), NoteDuration::new(1, 4).dotted(1)),
                    (String::from("1/2"), NoteDuration::new(1, 2)),
                    (String::from("1/2."), NoteDuration::new(1, 2).dotted(1)),
                    (String::from("1 bar"), NoteDuration::new(1, 1)),
                    (String::from("1 bar."), NoteDuration::new(1, 1).dotted(1)),
                    (String::from("2 bars"), NoteDuration::new(2, 1)),
                    (String::from("3 bars"), NoteDuration::new(3, 1)),
                ],
            ),
            feedback: PercentageParamBuilder::new("Feedback").build_float(),
        }
    }
}

impl RisDelay {}

impl Plugin for RisDelay {
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
        ProcessStatus::Normal
    }
}

impl ClapPlugin for RisDelay {
    const CLAP_ID: &'static str = "me.leiner.ris.delay";
    const CLAP_DESCRIPTION: Option<&'static str> = Some(env!("CARGO_PKG_DESCRIPTION"));
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] =
        &[ClapFeature::Custom("MIDI"), ClapFeature::Utility];
}

impl Vst3Plugin for RisDelay {
    const VST3_CLASS_ID: [u8; 16] = *b"risDelay........";
    const VST3_CATEGORIES: &'static str = "Fx|Tools";
}

nih_export_clap!(RisDelay);
nih_export_vst3!(RisDelay);

#[cfg(test)]
mod tests {
    use super::*;
}
