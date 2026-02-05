#[derive(Debug, Clone)]
pub struct EffectParameter {
    pub name: String,
    pub value: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub default_value: f32,
}

impl EffectParameter {
    pub fn new(name: &str, default: f32, min: f32, max: f32) -> Self {
        Self {
            name: name.to_string(),
            value: default,
            min_value: min,
            max_value: max,
            default_value: default,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min_value, self.max_value);
    }

    pub fn set_normalized(&mut self, normalized: f32) {
        let normalized = normalized.clamp(0.0, 1.0);
        self.value = self.min_value + normalized * (self.max_value - self.min_value);
    }

    pub fn get_normalized(&self) -> f32 {
        if self.max_value == self.min_value {
            0.0
        } else {
            (self.value - self.min_value) / (self.max_value - self.min_value)
        }
    }
}