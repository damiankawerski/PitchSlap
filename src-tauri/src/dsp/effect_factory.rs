use super::effect_trait::AudioEffect;


use once_cell::sync::Lazy;
use std::sync::Mutex;

use std::collections::HashMap;

pub type EffectFactory = fn() -> Box<dyn AudioEffect + Send + Sync>;

pub static EFFECTS: Lazy<Mutex<HashMap<String, EffectFactory>>> = Lazy::new(|| {
    let mut map = HashMap::new();

    fn testing_voice_factory() -> Box<dyn AudioEffect + Send + Sync> {
        Box::new(crate::dsp::effects::testing_effect::TestingVoice::new())
    }

    map.insert("TestingVoice".into(), testing_voice_factory as EffectFactory);
    Mutex::new(map)
});
