// Buffer to hold consumer and produder for audio data
// Made to single input - single output audio processing
// For loopback implementation you should create new Stream 

use ringbuf::{traits::Split, HeapRb};
use ringbuf::consumer::Consumer;
use ringbuf::producer::Producer;


// AudioBuffer struct to hold the audio data buffer
pub struct AudioBuffer {
    // Producer (used to write audio data)
    producer: <HeapRb<f32> as Split>::Prod,

    // Consumer (used to read audio data)
    consumer: <HeapRb<f32> as Split>::Cons,
}

impl AudioBuffer {
    // Constructor Args: buffer_size - size of the buffer in samples

    // Buffer size could be * 2 to secure that there is enough space for audio data with the cost of more latency
    pub fn new(buffer_size: usize) -> Self {
        let heap_rb = HeapRb::<f32>::new(buffer_size);

        let (mut producer, consumer) = heap_rb.split();

        // Initialize the producer with zeroes
        for _ in 0..buffer_size {
            producer.try_push(0.0).unwrap();
        }

        AudioBuffer {
            producer,
            consumer,
        }
    }

    // Write audio data to the buffer (tries to write all data from slice) (could add output fell behind check)
    pub fn buffer_write(&mut self, data: &[f32]) -> anyhow::Result<()> {
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

    // Read audio data from the buffer (tries to read all data into slice)
    pub fn buffer_read(&mut self, data: &mut [f32], input_channels: usize, output_channels: usize) -> anyhow::Result<()> {
        let mut input_fell_behind = false;
        
        // Use external channel conversion function
        convert_audio_channels(
            input_channels,
            output_channels,
            data,
            &mut || {
                match self.consumer.try_pop() {
                    Some(s) => s,
                    None => {
                        input_fell_behind = true;
                        0.0
                    }
                }
            }
        )?;
        
        if input_fell_behind {
            eprintln!("input stream fell behind: try increasing latency");
        }

        Ok(())
    }

}
    



fn convert_audio_channels<F>(
    input_channels: usize,
    output_channels: usize,
    data: &mut [f32],
    sample_provider: &mut F
) -> anyhow::Result<()>
where
    F: FnMut() -> f32,
{
    match (input_channels, output_channels) {
        (1, 1) => {
            // Mono to mono - direct copy
            for sample in data {
                *sample = sample_provider();
            }
        }
        (1, 2) => {
            // Mono to stereo - duplicate mono signal to both channels
            for chunk in data.chunks_exact_mut(2) {
                let mono_sample = sample_provider();
                chunk[0] = mono_sample; // Left channel
                chunk[1] = mono_sample; // Right channel
            }
        }
        (2, 1) => {
            // Stereo to mono - mix both channels
            for sample in data {
                let left = sample_provider();
                let right = sample_provider();
                *sample = (left + right) / 2.0; // Mix to mono
            }
        }
        (2, 2) => {
            // Stereo to stereo - direct copy
            for sample in data {
                *sample = sample_provider();
            }
        }
        _ => {
            // Unsupported channel configuration
            return Err(anyhow::anyhow!(
                "Unsupported channel configuration: {} input channels, {} output channels", 
                input_channels, 
                output_channels
            ));
        }
    }
    
    Ok(())
}



// To też powinno działać bosko