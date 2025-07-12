// Global engine 

use super::engine::AudioEngine;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread::{JoinHandle};

pub static AUDIO_ENGINE: OnceLock<Mutex<Option<AudioEngine>>> = OnceLock::new();
pub static LOOPBACK_THREAD: OnceLock<Arc<Mutex<Option<JoinHandle<Result<(), String>>>>>> = OnceLock::new();