use crate::dsp::traits::FilterModule;

#[derive(Clone)]
pub struct BiquadFilter {
    // Filter coefficients
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,

    // Filter memory
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

impl BiquadFilter {
    pub fn new() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    pub fn process_internal(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;

        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_coefficients(
        &mut self,
        b0: f32,
        b1: f32,
        b2: f32,
        a0: f32,
        a1: f32,
        a2: f32,
    ) {
        // Normalize by a0
        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }

    pub fn configure_peaking(
        &mut self,
        sample_rate: f32,
        frequency: f32,
        q: f32,
        gain_db: f32,
    ) {
        use std::f32::consts::PI;

        let omega = 2.0 * PI * frequency / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);
        let a = 10_f32.powf(gain_db / 40.0);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos_omega;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha / a;

        self.set_coefficients(b0, b1, b2, a0, a1, a2);
    }

    pub fn configure_lowpass(&mut self, sample_rate: f32, frequency: f32, q: f32) {
        use std::f32::consts::PI;

        let omega = 2.0 * PI * frequency / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let b0 = (1.0 - cos_omega) / 2.0;
        let b1 = 1.0 - cos_omega;
        let b2 = (1.0 - cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        self.set_coefficients(b0, b1, b2, a0, a1, a2);
    }

    pub fn configure_highpass(&mut self, sample_rate: f32, frequency: f32, q: f32) {
        use std::f32::consts::PI;

        let omega = 2.0 * PI * frequency / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let b0 = (1.0 + cos_omega) / 2.0;
        let b1 = -(1.0 + cos_omega);
        let b2 = (1.0 + cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        self.set_coefficients(b0, b1, b2, a0, a1, a2);
    }

    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }

    pub fn set_low_shelf(&mut self, frequency: f32, gain_db: f32, sample_rate: f32) {
        use std::f32::consts::{FRAC_1_SQRT_2, PI};

        let omega = 2.0 * PI * frequency / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let a = 10_f32.powf(gain_db / 40.0);
        let beta = a.sqrt() * FRAC_1_SQRT_2; // Q = 1/sqrt(2)

        let b0 = a * ((a + 1.0) - (a - 1.0) * cos_omega + beta * sin_omega);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_omega);
        let b2 = a * ((a + 1.0) - (a - 1.0) * cos_omega - beta * sin_omega);
        let a0 = (a + 1.0) + (a - 1.0) * cos_omega + beta * sin_omega;
        let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_omega);
        let a2 = (a + 1.0) + (a - 1.0) * cos_omega - beta * sin_omega;

        self.set_coefficients(b0, b1, b2, a0, a1, a2);
    }

    pub fn set_high_shelf(&mut self, frequency: f32, gain_db: f32, sample_rate: f32) {
        use std::f32::consts::{FRAC_1_SQRT_2, PI};

        let omega = 2.0 * PI * frequency / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let a = 10_f32.powf(gain_db / 40.0);
        let beta = a.sqrt() * FRAC_1_SQRT_2; // Q = 1/sqrt(2)

        let b0 = a * ((a + 1.0) + (a - 1.0) * cos_omega + beta * sin_omega);
        let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_omega);
        let b2 = a * ((a + 1.0) + (a - 1.0) * cos_omega - beta * sin_omega);
        let a0 = (a + 1.0) - (a - 1.0) * cos_omega + beta * sin_omega;
        let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_omega);
        let a2 = (a + 1.0) - (a - 1.0) * cos_omega - beta * sin_omega;

        self.set_coefficients(b0, b1, b2, a0, a1, a2);
    }

    pub fn set_peaking(&mut self, frequency: f32, gain_db: f32, q: f32, sample_rate: f32) {
        self.configure_peaking(sample_rate, frequency, q, gain_db);
    }
}

impl FilterModule for BiquadFilter {
    fn process(&mut self, input: f32) -> f32 {
        self.process_internal(input)
    }

    fn reset(&mut self) {
        self.reset();
    }
}