// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use pitchslap_lib::audio::globals::{AUDIO_ENGINE, LOOPBACK_THREAD};
// use pitchslap_lib::commands::loopback::{loopback, stop_loopback};


fn main() -> anyhow::Result<()> {
    
    pitchslap_lib::commands::app_runner::run();
    Ok(())
}

// use pitchslap_lib::audio::audio_handler::AudioHandler;
// use pitchslap_lib::audio::device::AudioDeviceOptions;
// use std::time::Duration;

// fn main() -> anyhow::Result<()> {
//     println!("Starting audio loopback...");

//     // Stwórz AudioHandler
//     let mut handler = AudioHandler::new();

//     // (Opcjonalnie) Ustaw custom audio devices
//     let options = AudioDeviceOptions::new(
//         "default".to_string(),                              // input device
//         "default".to_string(),                              // output device
//         "CABLE Input (VB-Audio Virtual Cable)".to_string(), // virtual input
//         150.0,                                              // latency
//     );
    
//     handler.select_audio_devices(&options)?;

//     // Uruchom loopback
//     handler.start_audio_engine_loopback()?;

//     // Sprawdź czy działa
//     if handler.is_running() {
//         println!("Audio loopback is running!");
        
//         // Pozostaw działające przez 10 sekund
//         std::thread::sleep(Duration::from_secs(10));
        
//         // Zatrzymaj
//         handler.stop_audio_engine_loopback()?;
//         println!("Audio loopback stopped");
//     }

//     Ok(())
// }


