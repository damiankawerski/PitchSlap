use std::sync::{Arc, Mutex};
use anyhow::Result;
use once_cell::sync::Lazy;
use crate::audio::audio_handler::*;
use crate::audio::device::*;

static AUDIO_HANDLER: Lazy<Arc<Mutex<Option<AudioHandler>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

pub fn loopback() -> Result<()> {
    println!("Starting audio loopback...");

    let mut handler = AudioHandler::new();

    let options = AudioDeviceOptions::new(
        "default".to_string(),
        "default".to_string(),
        "CABLE Input (VB-Audio Virtual Cable)".to_string(),
        5000.0,
    );

    handler.select_audio_devices(&options)?;
    handler.start_audio_engine_loopback()?;

    if handler.is_running() {
        println!("Audio loopback is running!");
        let mut locked = AUDIO_HANDLER.lock().unwrap();
        *locked = Some(handler);
    }

    Ok(())
}

pub fn stop_loopback() -> Result<()> {
    let mut locked = AUDIO_HANDLER.lock().unwrap();
    if let Some(handler) = locked.as_mut() {
        handler.stop_audio_engine_loopback()?;
        println!("Audio loopback stopped");
    } else {
        println!("No running loopback to stop");
    }

    *locked = None;

    Ok(())
}

pub fn throughput() -> Result<()> {
    println!("Starting audio throughput...");

    let mut handler = AudioHandler::new();

    let options = AudioDeviceOptions::new(
        "default".to_string(),
        "default".to_string(),
        "CABLE Input (VB-Audio Virtual Cable)".to_string(),
        150.0,
    );

    handler.select_audio_devices(&options)?;
    handler.start_audio_engine_throughput()?;

    if handler.is_running() {
        println!("Audio throughput is running!");
        let mut locked = AUDIO_HANDLER.lock().unwrap();
        *locked = Some(handler);
    }

    Ok(())
}

pub fn stop_throughput() -> Result<()> {
    let mut locked = AUDIO_HANDLER.lock().unwrap();
    if let Some(handler) = locked.as_mut() {
        handler.stop_audio_engine_throughput()?;
        println!("Audio throughput stopped");
    } else {
        println!("No running throughput to stop");
    }

    *locked = None;

    Ok(())
}