// src-tauri/src/audio/mod.rs
// Module for containing audio streams 


use ringbuf::{
    traits::{Split}, HeapRb,
};
use ringbuf::producer::Producer;
use ringbuf::consumer::Consumer;



pub struct AudioBuffer {
    // Producer for the audio buffer
    producer: <HeapRb<f32> as Split>::Prod,
    // Consumer for the audio buffer
    consumer: <HeapRb<f32> as Split>::Cons,
}



impl AudioBuffer {
    // Constructor for AudioBuffer
    pub fn new(capacity: usize) -> Self {
        let rb = HeapRb::<f32>::new(capacity * 2);
        let (mut producer, consumer) = rb.split();
        
        for _ in 0..capacity {
            producer.try_push(0.0).unwrap(); // Initialize buffer with zeros
        } 

        Self {
            producer,
            consumer,
        }
    }

    // Function to write audio data to the buffer
    pub fn input_data_fn(&mut self, data: &[f32], _:&cpal::InputCallbackInfo) -> anyhow::Result<()> {
        let mut output_fell_behind = false;
        for &sample in data {
            if self.producer.try_push(sample).is_err() {
                output_fell_behind = true;
                break; 
            }
        }
        if output_fell_behind {
            eprintln!("Output buffer fell behind, some samples were dropped.");
        }
        Ok(())
    }

    // Function to read audio data from the buffer with simple channel conversion
    pub fn output_data_fn(&mut self, input_channels: usize, output_channels: usize ,data: &mut [f32], _:&cpal::OutputCallbackInfo) -> anyhow::Result<()> {
        let mut input_fell_behind = false;
        

        // Handle channel conversion
        match (input_channels, output_channels) {
            (1, 1) => {
                // Mono to mono - direct copy
                for sample in data {
                    *sample = match self.consumer.try_pop() {
                        Some(s) => s,
                        None => {
                            input_fell_behind = true;
                            0.0
                        }
                    };
                }
            }
            (1, 2) => {
                // Mono to stereo - duplicate mono signal to both channels
                for chunk in data.chunks_exact_mut(2) {
                    let mono_sample = match self.consumer.try_pop() {
                        Some(s) => s,
                        None => {
                            input_fell_behind = true;
                            0.0
                        }
                    };
                    chunk[0] = mono_sample; // Left channel
                    chunk[1] = mono_sample; // Right channel
                }
            }
            (2, 1) => {
                // Stereo to mono - mix both channels
                for sample in data {
                    let left = match self.consumer.try_pop() {
                        Some(s) => s,
                        None => {
                            input_fell_behind = true;
                            0.0
                        }
                    };
                    let right = match self.consumer.try_pop() {
                        Some(s) => s,
                        None => {
                            input_fell_behind = true;
                            0.0
                        }
                    };
                    *sample = (left + right) / 2.0; // Mix to mono
                }
            }
            (2, 2) => {
                // Stereo to stereo - direct copy
                for sample in data {
                    *sample = match self.consumer.try_pop() {
                        Some(s) => s,
                        None => {
                            input_fell_behind = true; 
                            0.0
                        }
                    };
                }
            }
            _ => {
                // Unsupported channel configuration
                panic!("Unsupported channel configuration: {} input channels, {} output channels", input_channels, output_channels);
            }
        }
        
        if input_fell_behind {
            eprintln!("input stream fell behind: try increasing latency");
        }
        Ok(())
    }
}
