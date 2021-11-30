use std::collections::VecDeque;

use num_traits::Float;

mod function_macro;
pub mod functions;

pub struct Gain<'a, T>
where
    T: Float,
{
    gain: T,
    time: i32,
    duration: i32,
    easing_func: &'a dyn Fn(T) -> T,
}

impl<'a, T: Float> Gain<'a, T> {
    pub fn new(gain: T, time: i32, duration: i32, easing_func: &'a dyn Fn(T) -> T) -> Self {
        Self {
            gain,
            time,
            duration,
            easing_func,
        }
    }

    pub fn calc(&self, t: i32) -> T {
        println!("t: {}, time: {}, duration: {}", t, self.time, self.duration);
        let x = T::from(t - self.time).unwrap() / T::from(self.duration).unwrap();
        (self.easing_func)(x) * self.gain
    }
}
pub struct EasingValue<'a, T: Float> {
    value: T,
    time: i32,
    queue: Vec<Gain<'a, T>>,
}

impl<'a, T: Float> EasingValue<'a, T> {
    pub fn new(value: T, time: i32) -> Self {
        Self {
            value,
            time,
            queue: Vec::new(),
        }
    }

    pub fn add(&mut self, gain: Gain<'a, T>) {
        self.queue.push(gain);
    }

    pub fn current_value(&self, time: i32) -> T {
        let gain: T = self
            .queue
            .iter()
            .map(|gain| gain.calc(time))
            .fold(T::zero(), |sum, t| sum + t);
        self.value + gain
    }
}
