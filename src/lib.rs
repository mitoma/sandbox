use std::time::{Duration, SystemTime, UNIX_EPOCH};

use num_traits::{Float, ToPrimitive};

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

    pub fn calc(&self, time: i32) -> T {
        let x = T::from(time - self.time).unwrap() / T::from(self.duration).unwrap();
        (self.easing_func)(x) * self.gain
    }

    pub fn before(&self, time: i32) -> bool {
        self.time > time
    }

    pub fn after(&self, time: i32) -> bool {
        self.time + self.duration < time
    }
}

pub struct TimeBaseEasingValue<'a, T: Float>(EasingValue<'a, T>);

impl<'a, T: Float> TimeBaseEasingValue<'a, T> {
    pub fn new(value: T) -> Self {
        Self(EasingValue::new(value))
    }

    pub fn add(&mut self, gain: T, duration: Duration, easing_func: &'a dyn Fn(T) -> T) {
        self.0.add(Gain::new(
            gain,
            self.current_time(),
            duration.as_millis().to_i32().unwrap(),
            easing_func,
        ))
    }

    pub fn gc(&mut self) {
        self.0.gc(self.current_time());
    }

    pub fn current_value(&self) -> T {
        self.0.current_value(self.current_time())
    }

    pub fn in_animation(&self) -> bool {
        self.0.in_animation(self.current_time())
    }

    #[inline]
    fn current_time(&self) -> i32 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_i32()
            .unwrap()
    }
}

pub struct EasingValue<'a, T: Float> {
    value: T,
    queue: Vec<Gain<'a, T>>,
}

impl<'a, T: Float> EasingValue<'a, T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            queue: Vec::new(),
        }
    }

    pub fn add(&mut self, gain: Gain<'a, T>) {
        self.queue.push(gain);
    }

    pub fn gc(&mut self, time: i32) {
        let gain: T = self
            .queue
            .iter()
            .filter(|gain| gain.after(time))
            .map(|gain| gain.calc(time))
            .fold(T::zero(), |sum, t| sum + t);

        self.value = self.value + gain;
        self.queue.retain(|gain| gain.after(time));
    }

    pub fn current_value(&self, time: i32) -> T {
        let gain: T = self
            .queue
            .iter()
            .map(|gain| gain.calc(time))
            .fold(T::zero(), |sum, t| sum + t);
        self.value + gain
    }

    pub fn in_animation(&self, time: i32) -> bool {
        self.queue.iter().any(|gain| gain.after(time))
    }
}
