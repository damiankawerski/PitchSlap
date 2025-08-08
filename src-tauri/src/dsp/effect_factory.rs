use super::effect_trait::AudioEffect;
use super::effects::anime_voice::AnimeVoice;
use super::effects::robo_voice::RoboVoice;
use super::effects::reverb::Reverb;

use once_cell::sync::Lazy;
use std::sync::Mutex;

use std::collections::HashMap;

pub type EffectFactory = fn() -> Box<dyn AudioEffect + Send + Sync>;

pub static EFFECTS: Lazy<Mutex<HashMap<String, EffectFactory>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    fn anime_voice_factory() -> Box<dyn AudioEffect + Send + Sync> {
        Box::new(AnimeVoice::new())
    }
    fn robo_voice_factory() -> Box<dyn AudioEffect + Send + Sync> {
        Box::new(RoboVoice::new())
    }
    fn reverb_factory() -> Box<dyn AudioEffect + Send + Sync> {
        Box::new(Reverb::new(0.5))
    }
    fn chorus_factory() -> Box<dyn AudioEffect + Send + Sync> {
        Box::new(crate::dsp::effects::chorus::Chorus::new(0.5))
    }
    fn chipmunk_factory() -> Box<dyn AudioEffect + Send + Sync> {
        Box::new(crate::dsp::effects::chipmunk::ChipmunkVoice::new())
    }
    fn demon_factory() -> Box<dyn AudioEffect + Send + Sync> {
        Box::new(crate::dsp::effects::demon::DemonVoice::new())
    }
    map.insert("Chorus".into(), chorus_factory as EffectFactory);
    map.insert("ChipmunkVoice".into(), chipmunk_factory as EffectFactory);
    map.insert("DemonVoice".into(), demon_factory as EffectFactory);
    map.insert("AnimeVoice".into(), anime_voice_factory as EffectFactory);
    map.insert("RoboVoice".into(), robo_voice_factory as EffectFactory);
    map.insert("Reverb".into(), reverb_factory as EffectFactory);
    Mutex::new(map)
});
