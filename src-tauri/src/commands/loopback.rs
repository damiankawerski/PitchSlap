// Testing loopback for the audio engine in app
// THIS IS A TESTTING FILE

use crate::audio::engine::AudioEngine;
use crate::audio::globals::{AUDIO_ENGINE, LOOPBACK_THREAD};
use std::sync::{Arc, Mutex};
use std::thread;

#[tauri::command]
pub fn loopback() -> Result<(), String> {
    // Sprawdź czy wątek już działa
    let thread_holder = LOOPBACK_THREAD.get_or_init(|| Arc::new(Mutex::new(None)));
    {
        let guard = thread_holder.lock().unwrap();
        if guard.is_some() {
            return Ok(()); // Już działa
        }
    }

    // Spawn nowego wątku dla loopback
    let handle = thread::spawn(move || -> Result<(), String> {
        // Zainicjalizuj singleton w wątku
        let engine_lock = AUDIO_ENGINE.get_or_init(|| Mutex::new(None));
        let mut guard = engine_lock.lock().unwrap();

        // Jeśli nie ma silnika, twórz
        if guard.is_none() {
            let engine = AudioEngine::new().unwrap();
            engine.start_input_stream().map_err(|e| e.to_string())?;
            engine.start_output_stream().map_err(|e| e.to_string())?;
            *guard = Some(engine);
        }

        // Zwolnij lock żeby inne wątki mogły używać silnika
        drop(guard);

        // Trzymaj wątek przy życiu - czekaj na signal do zatrzymania
        loop {
            thread::park();
            
            // Sprawdź czy powinniśmy się zatrzymać
            if let Some(engine_lock) = AUDIO_ENGINE.get() {
                let guard = engine_lock.lock().unwrap();
                if guard.is_none() {
                    break; // Silnik został zatrzymany
                }
            }
        }

        Ok(())
    });

    // Zapisz handle wątku
    {
        let mut guard = thread_holder.lock().unwrap();
        *guard = Some(handle);
    }

    Ok(())
}

#[tauri::command]
pub fn stop_loopback() -> Result<(), String> {
    // Zatrzymaj silnik
    if let Some(engine_lock) = AUDIO_ENGINE.get() {
        let mut guard = engine_lock.lock().unwrap();

        if let Some(engine) = guard.take() {
            engine.stop_input_stream().map_err(|e| e.to_string())?;
            engine.stop_output_stream().map_err(|e| e.to_string())?;
        }
    }

    // Rozbudź i poczekaj na zakończenie wątku
    if let Some(thread_holder) = LOOPBACK_THREAD.get() {
        let mut guard = thread_holder.lock().unwrap();
        if let Some(handle) = guard.take() {
            // Najpierw rozbudź wątek
            handle.thread().unpark();
            
            // Zwolnij lock przed join
            drop(guard);
            
            // Poczekaj na zakończenie wątku
            if let Err(e) = handle.join() {
                eprintln!("Error joining loopback thread: {:?}", e);
            }
        }
    }

    Ok(())
}