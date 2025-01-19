use fundsp::hacker::*;
use nih_plug::prelude::*;

use std::sync::Arc;

struct Chord {
    params: Arc<GainParams>,
    patch: Option<Box<dyn AudioUnit>>,
}

#[derive(Params)]
struct GainParams {
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for Chord {
    fn default() -> Self {
        Self {
            params: Arc::new(GainParams::default()),
            patch: None,
        }
    }
}

impl Default for GainParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for Chord {
    const NAME: &'static str = "Organ patch";
    const VENDOR: &'static str = "Plugggs GmbH";
    const URL: &'static str = "";
    const EMAIL: &'static str = "info@example.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        let c = 0.2 * (organ_hz(midi_hz(57.0)) + organ_hz(midi_hz(61.0)) + organ_hz(midi_hz(64.0)));
        let c = c >> pan(0.0);

        // Add chorus.
        let c = c >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));

        let mut c =
            c >> (declick() | declick()) >> (dcblock() | dcblock()) >> limiter_stereo(1.0, 5.0);

        c.set_sample_rate(buffer_config.sample_rate.into());
        c.allocate();

        self.patch = Some(Box::new(c));
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let out = buffer.as_slice(); // two slices, one for each channel
        let frame_len = out[0].len();

        if let Some(patch) = &mut self.patch {
            for i in 0..frame_len {
                let gain = self.params.gain.smoothed.next();

                let (in_l, in_r) = patch.get_stereo();
                out[0][i] = in_l * gain;
                out[1][i] = in_r * gain;
            }
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl ClapPlugin for Chord {
    const CLAP_ID: &'static str = "com.plugggs-gmbh.nih_plug-organ";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for Chord {
    const VST3_CLASS_ID: [u8; 16] = *b"$$$$Plugggs_$$$$";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(Chord);
