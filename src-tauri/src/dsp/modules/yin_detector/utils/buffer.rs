use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use std::sync::{Arc, Mutex};

use crate::dsp::modules::yin_detector::float::Float;

pub enum ComplexComponent {
    Re,
    Im,
}

pub fn new_real_buffer<T: Float>(size: usize) -> Vec<T> {
    vec![T::zero(); size]
}

pub fn new_complex_buffer<T: Float>(size: usize) -> Vec<Complex<T>> {
    vec![Complex::zero(); size]
}

pub fn copy_real_to_complex<T: Float>(
    input: &[T],
    output: &mut [Complex<T>],
    component: ComplexComponent,
) {
    assert!(input.len() <= output.len());
    match component {
        ComplexComponent::Re => input.iter().zip(output.iter_mut()).for_each(|(i, o)| {
            o.re = *i;
            o.im = T::zero();
        }),
        ComplexComponent::Im => input.iter().zip(output.iter_mut()).for_each(|(i, o)| {
            o.im = *i;
            o.re = T::zero();
        }),
    }
    output[input.len()..]
        .iter_mut()
        .for_each(|o| *o = Complex::zero())
}

pub fn copy_complex_to_real<T: Float>(
    input: &[Complex<T>],
    output: &mut [T],
    component: ComplexComponent,
) {
    assert!(input.len() <= output.len());
    match component {
        ComplexComponent::Re => input
            .iter()
            .map(|c| c.re)
            .zip(output.iter_mut())
            .for_each(|(i, o)| *o = i),
        ComplexComponent::Im => input
            .iter()
            .map(|c| c.im)
            .zip(output.iter_mut())
            .for_each(|(i, o)| *o = i),
    }

    output[input.len()..]
        .iter_mut()
        .for_each(|o| *o = T::zero());
}

/// Computes |x|^2 for each complex value x in `arr`. This function
/// modifies `arr` in place and leaves the complex component zero.
pub fn modulus_squared<'a, T: Float>(arr: &'a mut [Complex<T>]) {
    for s in arr {
        s.re = s.re * s.re + s.im * s.im;
        s.im = T::zero();
    }
}

/// Compute the sum of the square of each element of `arr`.
pub fn square_sum<T>(arr: &[T]) -> T
where
    T: Float + std::iter::Sum,
{
    arr.iter().map(|&s| s * s).sum::<T>()
}

#[derive(Debug)]
pub struct BufferPool<T> {
    real_buffers: Vec<Arc<Mutex<Vec<T>>>>,
    complex_buffers: Vec<Arc<Mutex<Vec<Complex<T>>>>>,
    pub buffer_size: usize,
}

impl<T> Clone for BufferPool<T> {
    fn clone(&self) -> Self {
        Self {
            real_buffers: self.real_buffers.clone(),
            complex_buffers: self.complex_buffers.clone(),
            buffer_size: self.buffer_size,
        }
    }
}

impl<T: Float> BufferPool<T> {
    pub fn new(buffer_size: usize) -> Self {
        BufferPool {
            real_buffers: vec![],
            complex_buffers: vec![],
            buffer_size,
        }
    }
    fn add_real_buffer(&mut self) -> Arc<Mutex<Vec<T>>> {
        self.real_buffers
            .push(Arc::new(Mutex::new(new_real_buffer::<T>(
                self.buffer_size,
            ))));
        Arc::clone(&self.real_buffers.last().unwrap())
    }
    fn add_complex_buffer(&mut self) -> Arc<Mutex<Vec<Complex<T>>>> {
        self.complex_buffers
            .push(Arc::new(Mutex::new(new_complex_buffer::<T>(
                self.buffer_size,
            ))));
        Arc::clone(&self.complex_buffers.last().unwrap())
    }
    /// Get a reference to a buffer that can e used until it is `Drop`ed. Call
    /// `.lock()` to get a reference to a mutable version of the buffer.
    pub fn get_real_buffer(&mut self) -> Arc<Mutex<Vec<T>>> {
        self.real_buffers
            .iter()
            // If the Arc count is 1, we haven't loaned the buffer out yet.
            .find(|&buf| Arc::strong_count(buf) == 1)
            .map(|buf| Arc::clone(buf))
            // If we haven't found a buffer we can reuse, create one.
            .unwrap_or_else(|| self.add_real_buffer())
    }
    /// Get a reference to a buffer that can e used until it is `Drop`ed. Call
    /// `.lock()` to get a reference to a mutable version of the buffer.
    pub fn get_complex_buffer(&mut self) -> Arc<Mutex<Vec<Complex<T>>>> {
        self.complex_buffers
            .iter()
            // If the Arc count is 1, we haven't loaned the buffer out yet.
            .find(|&buf| Arc::strong_count(buf) == 1)
            .map(|buf| Arc::clone(buf))
            // If we haven't found a buffer we can reuse, create one.
            .unwrap_or_else(|| self.add_complex_buffer())
    }
}

#[test]
fn test_buffers() {
    let mut buffers = BufferPool::new(3);
    let buf_cell1 = buffers.get_real_buffer();
    {
        // This buffer won't be dropped until the end of the function
        let mut buf1 = buf_cell1.lock().unwrap();
        buf1[0] = 5.5;
    }
    {
        // This buffer will be dropped when the scope ends
        let buf_cell2 = buffers.get_real_buffer();
        let mut buf2 = buf_cell2.lock().unwrap();
        buf2[1] = 6.6;
    }
    {
        // This buffer will be dropped when the scope ends
        // It is the same buffer that was just used (i.e., it's a reused buffer)
        let buf_cell3 = buffers.get_real_buffer();
        let mut buf3 = buf_cell3.lock().unwrap();
        buf3[2] = 7.7;
    }
    // We're peering into the internals of `BufferPool`. This shouldn't normally be done.
    assert_eq!(&buffers.real_buffers[0].lock().unwrap()[..], &[5.5, 0., 0.]);
    assert_eq!(&buffers.real_buffers[1].lock().unwrap()[..], &[0.0, 6.6, 7.7]);
}
