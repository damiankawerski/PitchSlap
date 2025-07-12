// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use pitchslap_lib::audio::device::AudioDeviceOpt;
use pitchslap_lib::audio::config::{self, AudioDeviceConfig};
use pitchslap_lib::audio::engine::{loopback, AudioStreams};
use pitchslap_lib::audio::utils::*;


fn main() {

    // testing the audio engine
    loopback()
        .expect("Failed to start audio loopback");

    
    // pitchslap_lib::run()
} 
