use crate::dsp::modules::filters::BandPassFilter;
use crate::dsp::modules::effects::Reverb;
use crate::dsp::modules::utils::oscilator::Oscillator;
use crate::dsp::traits::{EffectModule, FilterModule};


// This was implemented BY AI
pub struct Vocoder {
	name: String,
	sample_rate: f32,
	band_count: usize,
	min_freq: f32,
	max_freq: f32,
	q: f32,
	attack_ms: f32,
	release_ms: f32,
	output_gain: f32,
	dry_mix: f32,
	env_gain: f32,
	env_floor: f32,
	soft_clip: f32,
	reverb_mix: f32,
	reverb: Reverb,
	reverb_buffer: Vec<f32>,
	carrier_base_freq: f32,
	carrier_harmonics: usize,
	carrier_gain: f32,
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
			band_count: 32,
			min_freq: 90.0,
			max_freq: 9_000.0,
			q: 9.0,
			attack_ms: 2.0,
			release_ms: 50.0,
			output_gain: 4.0,
			dry_mix: 0.2,
			env_gain: 11.0,
			env_floor: 0.0015,
			soft_clip: 1.6,
			reverb_mix: 0.08,
			reverb: Reverb::new(sample_rate as u32, 1),
			reverb_buffer: Vec::new(),
			carrier_base_freq: 110.0,
			carrier_harmonics: 18,
			carrier_gain: 0.22,
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
		effect.band_count = 32;
		effect.min_freq = 80.0;
		effect.max_freq = 9_000.0;
		effect.q = 10.0;
		effect.attack_ms = 2.5;
		effect.release_ms = 70.0;
		effect.output_gain = 5.5;
		effect.env_gain = 12.0;
		effect.env_floor = 0.0025;
		effect.soft_clip = 1.8;
		effect.reverb_mix = 0.1;
		effect.carrier_base_freq = 300.0;
		effect.carrier_harmonics = 20;
		effect.carrier_gain = 0.22;
		effect.rebuild();
		effect
	}

	fn rebuild(&mut self) {
		self.update_env_coeffs();
		self.rebuild_filters();
		self.rebuild_carrier();
	}

	fn update_env_coeffs(&mut self) {
		let attack = (self.attack_ms.max(0.1)) * 0.001;
		let release = (self.release_ms.max(1.0)) * 0.001;
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
			let bandwidth = (center / self.q).max(bandwidth_floor);
			self.mod_filters
				.push(BandPassFilter::new(self.sample_rate, center, bandwidth));
			self.car_filters
				.push(BandPassFilter::new(self.sample_rate, center, bandwidth));
			self.envelopes.push(0.0);
		}
	}

	fn rebuild_carrier(&mut self) {
		self.carrier_oscillators.clear();
		let harmonics = self.carrier_harmonics.max(1);
		for i in 1..=harmonics {
			let freq = self.carrier_base_freq * i as f32;
			let amp = self.carrier_gain / i as f32;
			self.carrier_oscillators.push(Oscillator::new(freq, amp));
		}
	}

	fn center_frequencies(&self) -> Vec<f32> {
		let bands = self.band_count.max(1);
		if bands == 1 {
			return vec![(self.min_freq + self.max_freq) * 0.5];
		}

		let ratio = (self.max_freq / self.min_freq).powf(1.0 / (bands as f32 - 1.0));
		let mut centers = Vec::with_capacity(bands);
		let mut freq = self.min_freq;
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
				let env = (env * self.env_gain + self.env_floor).clamp(0.0, 1.0);
				let filtered_carrier = self.car_filters[band].process(carrier_sample);
				acc += filtered_carrier * env * band_scale;
			}

			let mixed = acc * self.output_gain + mod_sample * self.dry_mix;
			output[i] = if self.soft_clip > 0.0 {
				(mixed * self.soft_clip).tanh()
			} else {
				mixed
			};
		}

		if self.reverb_mix > 0.0 {
			if self.reverb_buffer.len() != len {
				self.reverb_buffer.resize(len, 0.0);
			}
			self.reverb.process(&output[..len], &mut self.reverb_buffer);
			let wet = self.reverb_mix.clamp(0.0, 1.0);
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
}
