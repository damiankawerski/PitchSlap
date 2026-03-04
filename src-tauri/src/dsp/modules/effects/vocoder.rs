use crate::dsp::modules::filters::BandPassFilter;
use crate::dsp::modules::effects::Reverb;
use crate::dsp::modules::utils::{EffectParameter, ParameterValue};
use crate::dsp::modules::utils::oscilator::Oscillator;
use crate::dsp::traits::{EffectModule, FilterModule};


// This was implemented BY AI
pub struct Vocoder {
	name: String,
	sample_rate: f32,
	band_count: EffectParameter,
	min_freq: EffectParameter,
	max_freq: EffectParameter,
	q: EffectParameter,
	attack_ms: EffectParameter,
	release_ms: EffectParameter,
	output_gain: EffectParameter,
	dry_mix: EffectParameter,
	env_gain: EffectParameter,
	env_floor: EffectParameter,
	soft_clip: EffectParameter,
	reverb_mix: EffectParameter,
	reverb: Reverb,
	reverb_buffer: Vec<f32>,
	carrier_base_freq: EffectParameter,
	carrier_harmonics: EffectParameter,
	carrier_gain: EffectParameter,
	mod_filters: Vec<BandPassFilter>,
	car_filters: Vec<BandPassFilter>,
	envelopes: Vec<f32>,
	carrier_oscillators: Vec<Oscillator>,
	attack_coeff: f32,
	release_coeff: f32,
}

impl Vocoder {
	pub fn new(sample_rate: usize) -> Self {
		let mut effect = Self {
			name: "vocoder".to_string(),
			sample_rate: sample_rate as f32,
			band_count: EffectParameter::new("band_count", 32.0, 1.0, 64.0),
			min_freq: EffectParameter::new("min_freq", 90.0, 20.0, 4_000.0),
			max_freq: EffectParameter::new("max_freq", 9_000.0, 1_000.0, 20_000.0),
			q: EffectParameter::new("q", 9.0, 0.5, 30.0),
			attack_ms: EffectParameter::new("attack_ms", 2.0, 0.1, 200.0),
			release_ms: EffectParameter::new("release_ms", 50.0, 1.0, 500.0),
			output_gain: EffectParameter::new("output_gain", 4.0, 0.0, 20.0),
			dry_mix: EffectParameter::new("dry_mix", 0.2, 0.0, 1.0),
			env_gain: EffectParameter::new("env_gain", 11.0, 0.0, 30.0),
			env_floor: EffectParameter::new("env_floor", 0.0015, 0.0, 0.1),
			soft_clip: EffectParameter::new("soft_clip", 1.6, 0.0, 4.0),
			reverb_mix: EffectParameter::new("reverb_mix", 0.08, 0.0, 1.0),
			reverb: Reverb::new(sample_rate as u32, 1),
			reverb_buffer: Vec::new(),
			carrier_base_freq: EffectParameter::new("carrier_base_freq", 110.0, 20.0, 2_000.0),
			carrier_harmonics: EffectParameter::new("carrier_harmonics", 18.0, 1.0, 64.0),
			carrier_gain: EffectParameter::new("carrier_gain", 0.22, 0.0, 1.0),
			mod_filters: Vec::new(),
			car_filters: Vec::new(),
			envelopes: Vec::new(),
			carrier_oscillators: Vec::new(),
			attack_coeff: 0.0,
			release_coeff: 0.0,
		};

		effect.rebuild();
		effect
	}

	pub fn daft_punk(sample_rate: usize) -> Self {
		let mut effect = Self::new(sample_rate);
		effect.band_count.set_value(32.0);
		effect.min_freq.set_value(80.0);
		effect.max_freq.set_value(9_000.0);
		effect.q.set_value(10.0);
		effect.attack_ms.set_value(2.5);
		effect.release_ms.set_value(70.0);
		effect.output_gain.set_value(5.5);
		effect.env_gain.set_value(12.0);
		effect.env_floor.set_value(0.0025);
		effect.soft_clip.set_value(1.8);
		effect.reverb_mix.set_value(0.1);
		effect.carrier_base_freq.set_value(300.0);
		effect.carrier_harmonics.set_value(20.0);
		effect.carrier_gain.set_value(0.22);
		effect.rebuild();
		effect
	}

	fn rebuild(&mut self) {
		self.update_env_coeffs();
		self.rebuild_filters();
		self.rebuild_carrier();
	}

	fn update_env_coeffs(&mut self) {
		let attack = self.attack_ms.value * 0.001;
		let release = self.release_ms.value * 0.001;
		self.attack_coeff = (-1.0 / (attack * self.sample_rate)).exp();
		self.release_coeff = (-1.0 / (release * self.sample_rate)).exp();
	}

	fn rebuild_filters(&mut self) {
		let centers = self.center_frequencies();
		let bandwidth_floor = 10.0;

		self.mod_filters.clear();
		self.car_filters.clear();
		self.envelopes.clear();

		for center in centers {
			let bandwidth = (center / self.q.value).max(bandwidth_floor);
			self.mod_filters
				.push(BandPassFilter::new(self.sample_rate, center, bandwidth));
			self.car_filters
				.push(BandPassFilter::new(self.sample_rate, center, bandwidth));
			self.envelopes.push(0.0);
		}
	}

	fn rebuild_carrier(&mut self) {
		self.carrier_oscillators.clear();
		let harmonics = self.carrier_harmonics.value.round() as usize;
		for i in 1..=harmonics {
			let freq = self.carrier_base_freq.value * i as f32;
			let amp = self.carrier_gain.value / i as f32;
			self.carrier_oscillators.push(Oscillator::new(freq, amp));
		}
	}

	fn center_frequencies(&self) -> Vec<f32> {
		let bands = self.band_count.value.round() as usize;
		if bands == 1 {
			return vec![(self.min_freq.value + self.max_freq.value) * 0.5];
		}

		let ratio = (self.max_freq.value / self.min_freq.value).powf(1.0 / (bands as f32 - 1.0));
		let mut centers = Vec::with_capacity(bands);
		let mut freq = self.min_freq.value;
		for _ in 0..bands {
			centers.push(freq);
			freq *= ratio;
		}
		centers
	}

	fn next_carrier_sample(&mut self) -> f32 {
		let mut sample = 0.0;
		for osc in &mut self.carrier_oscillators {
			sample += osc.process(self.sample_rate);
		}
		sample
	}

	fn update_envelope(&mut self, index: usize, input: f32) -> f32 {
		let env = &mut self.envelopes[index];
		if input > *env {
			*env = input + (*env - input) * self.attack_coeff;
		} else {
			*env = input + (*env - input) * self.release_coeff;
		}
		*env
	}
}

impl EffectModule for Vocoder {
	fn process(&mut self, input: &[f32], output: &mut [f32]) {
		let len = input.len().min(output.len());
		if len == 0 {
			return;
		}
		let band_scale = 1.0 / (self.mod_filters.len() as f32).sqrt().max(1.0);

		for out in &mut output[..len] {
			*out = 0.0;
		}

		for i in 0..len {
			let mod_sample = input[i];
			let carrier_sample = self.next_carrier_sample();
			let mut acc = 0.0;

			for band in 0..self.mod_filters.len() {
				let filtered_mod = self.mod_filters[band].process(mod_sample);
				let env = self.update_envelope(band, filtered_mod.abs());
				let env = (env * self.env_gain.value + self.env_floor.value).clamp(0.0, 1.0);
				let filtered_carrier = self.car_filters[band].process(carrier_sample);
				acc += filtered_carrier * env * band_scale;
			}

			let mixed = acc * self.output_gain.value + mod_sample * self.dry_mix.value;
			output[i] = if self.soft_clip.value > 0.0 {
				(mixed * self.soft_clip.value).tanh()
			} else {
				mixed
			};
		}

		if self.reverb_mix.value > 0.0 {
			if self.reverb_buffer.len() != len {
				self.reverb_buffer.resize(len, 0.0);
			}
			self.reverb.process(&output[..len], &mut self.reverb_buffer);
			let wet = self.reverb_mix.value.clamp(0.0, 1.0);
			let dry = 1.0 - wet;
			for i in 0..len {
				output[i] = output[i] * dry + self.reverb_buffer[i] * wet;
			}
		}
	}

	fn reset(&mut self) {
		for filter in &mut self.mod_filters {
			filter.reset();
		}
		for filter in &mut self.car_filters {
			filter.reset();
		}
		for env in &mut self.envelopes {
			*env = 0.0;
		}
		for osc in &mut self.carrier_oscillators {
			osc.reset();
		}
		self.reverb.reset();
	}

	fn name(&self) -> &str {
		&self.name
	}

	fn set_parameter(&mut self, parameter: ParameterValue) -> anyhow::Result<()> {
		match parameter.name.as_str() {
			"band_count" => {
				self.band_count.set_value(parameter.value);
				self.rebuild_filters();
				Ok(())
			}
			"min_freq" => {
				self.min_freq.set_value(parameter.value);
				self.rebuild_filters();
				Ok(())
			}
			"max_freq" => {
				self.max_freq.set_value(parameter.value);
				self.rebuild_filters();
				Ok(())
			}
			"q" => {
				self.q.set_value(parameter.value);
				self.rebuild_filters();
				Ok(())
			}
			"attack_ms" => {
				self.attack_ms.set_value(parameter.value);
				self.update_env_coeffs();
				Ok(())
			}
			"release_ms" => {
				self.release_ms.set_value(parameter.value);
				self.update_env_coeffs();
				Ok(())
			}
			"output_gain" => {
				self.output_gain.set_value(parameter.value);
				Ok(())
			}
			"dry_mix" => {
				self.dry_mix.set_value(parameter.value);
				Ok(())
			}
			"env_gain" => {
				self.env_gain.set_value(parameter.value);
				Ok(())
			}
			"env_floor" => {
				self.env_floor.set_value(parameter.value);
				Ok(())
			}
			"soft_clip" => {
				self.soft_clip.set_value(parameter.value);
				Ok(())
			}
			"reverb_mix" => {
				self.reverb_mix.set_value(parameter.value);
				Ok(())
			}
			"carrier_base_freq" => {
				self.carrier_base_freq.set_value(parameter.value);
				self.rebuild_carrier();
				Ok(())
			}
			"carrier_harmonics" => {
				self.carrier_harmonics.set_value(parameter.value);
				self.rebuild_carrier();
				Ok(())
			}
			"carrier_gain" => {
				self.carrier_gain.set_value(parameter.value);
				self.rebuild_carrier();
				Ok(())
			}
			_ => Err(anyhow::anyhow!("Unknown parameter: {}", parameter.name)),
		}
	}

	fn get_parameters(&self) -> Vec<EffectParameter> {
		vec![
			self.band_count.clone(),
			self.min_freq.clone(),
			self.max_freq.clone(),
			self.q.clone(),
			self.attack_ms.clone(),
			self.release_ms.clone(),
			self.output_gain.clone(),
			self.dry_mix.clone(),
			self.env_gain.clone(),
			self.env_floor.clone(),
			self.soft_clip.clone(),
			self.reverb_mix.clone(),
			self.carrier_base_freq.clone(),
			self.carrier_harmonics.clone(),
			self.carrier_gain.clone(),
		]
	}
}
