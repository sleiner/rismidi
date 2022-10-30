use nih_plug::prelude::*;
use std::sync::Arc;

struct NoGui {
    params: Arc<NoGuiParams>,
}

#[derive(Params)]
struct NoGuiParams {
    #[id = "float"]
    pub float: FloatParam,

    #[id = "integer"]
    pub integer: IntParam,

    #[id = "char"]
    pub char: EnumParam<UpperChar>,
}

#[derive(Enum, PartialEq, Eq, Debug)]
enum UpperChar {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Default for NoGui {
    fn default() -> Self {
        Self {
            params: Arc::new(NoGuiParams::default()),
        }
    }
}

impl Default for NoGuiParams {
    fn default() -> Self {
        Self {
            float: FloatParam::new(
                "Float",
                0f32,
                FloatRange::Linear {
                    min: 0f32,
                    max: 1f32,
                },
            ),
            integer: IntParam::new("Int", 1, IntRange::Linear { min: 1, max: 100 }),
            char: EnumParam::new("Character", UpperChar::A),
        }
    }
}

impl Plugin for NoGui {
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const VENDOR: &'static str = "Simon Leiner";
    const EMAIL: &'static str = "rismidi@leiner.me";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const DEFAULT_INPUT_CHANNELS: u32 = 0;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 0;

    // const DEFAULT_AUX_INPUTS: Option<AuxiliaryIOConfig> = None;
    // const DEFAULT_AUX_OUTPUTS: Option<AuxiliaryIOConfig> = None;

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
        _: &mut Buffer,
        _: &mut AuxiliaryBuffers,
        _: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        ProcessStatus::Normal
    }
}

impl ClapPlugin for NoGui {
    const CLAP_ID: &'static str = "me.leiner.clap.nogui";
    const CLAP_DESCRIPTION: Option<&'static str> = Some(env!("CARGO_PKG_DESCRIPTION"));
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for NoGui {
    const VST3_CLASS_ID: [u8; 16] = *b"nogui...........";
    const VST3_CATEGORIES: &'static str = "Fx|Tools";
}

nih_export_clap!(NoGui);
nih_export_vst3!(NoGui);
