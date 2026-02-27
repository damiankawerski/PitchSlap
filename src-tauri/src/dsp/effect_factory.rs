use crate::dsp::modules::effects::{
	Amplifier, AutoTune, Bitcrusher, Chorus, Distortion, PitchShifter, Reverb, Scale, Vibrato,
	Vocoder,
};
use crate::dsp::traits::EffectModule;

fn normalize_effect_name(name: &str) -> String {
	name.trim().to_ascii_lowercase().replace(['-', ' '], "_")
}

pub fn create_effect_from_name(
	name: &str,
	sample_rate: usize,
	channels: usize,
) -> Result<Box<dyn EffectModule>, String> {
	let normalized = normalize_effect_name(name);
	let channels = channels.max(1);

	let effect: Box<dyn EffectModule> = match normalized.as_str() {
		"amplifier" | "amp" => Box::new(Amplifier::new(1.2)),
		"distortion" | "drive" => Box::new(Distortion::new(8.0)),
		"bitcrusher" | "bit_crusher" => Box::new(Bitcrusher::new(8.0, 4)),
		"chorus" => Box::new(Chorus::new(sample_rate, 0.35, 0.45)),
		"vibrato" => Box::new(Vibrato::new(5.0, 0.6, 0.5, sample_rate as f32)),
		"pitch_shifter" | "pitchshifter" | "pitch" => {
			Box::new(PitchShifter::new(30, sample_rate, 0.0, 8))
		}
		"auto_tune" | "autotune" => {
			let mut auto_tune = AutoTune::new(sample_rate as f32);
			auto_tune.set_scale(Scale::CMajor);
			auto_tune.set_correction_speed(0.95);
			auto_tune.set_max_sustain(200.0);
			auto_tune.set_power_threshold(0.05);
			auto_tune.set_clarity_threshold(0.30);
			Box::new(auto_tune)
		}
		"reverb" => {
			let mut reverb = Reverb::new(sample_rate as u32, channels);
			reverb.set_room_size(0.5);
			reverb.set_damping(0.5);
			reverb.set_wet_level(0.30);
			reverb.set_dry_level(0.70);
			Box::new(reverb)
		}
		"vocoder" => Box::new(Vocoder::new(sample_rate)),
		"vocoder_daft_punk" | "daft_punk" => Box::new(Vocoder::daft_punk(sample_rate)),
		_ => {
			return Err(format!(
				"Unknown effect name: '{}'. Supported effects: amplifier, distortion, bitcrusher, chorus, vibrato, pitch_shifter, auto_tune, reverb, vocoder",
				name
			))
		}
	};

	Ok(effect)
}

