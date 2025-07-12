// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use pitchslap_lib::audio::globals::{AUDIO_ENGINE, LOOPBACK_THREAD};
// use pitchslap_lib::commands::loopback::{loopback, stop_loopback};

fn main() {


    
    pitchslap_lib::commands::app_runner::run();
} 
