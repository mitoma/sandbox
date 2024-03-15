use instant::{Duration, SystemTime};
use num_traits::{Float, ToPrimitive};

pub mod array;
mod function_macro;
pub mod functions;

pub struct Gain<T>
where
    T: Float,
{
    gain: T,
    time: i64,
    duration: i64,
    easing_func: fn(T) -> T,
}

impl<T: Float> Gain<T> {
    pub fn new(gain: T, time: i64, duration: i64, easing_func: fn(T) -> T) -> Self {
        Self {
            gain,
            time,
            duration,
            easing_func,
        }
    }

    pub fn calc(&self, time: i64) -> T {
        let x = T::from(time - self.time).unwrap() / T::from(self.duration).unwrap();
        (self.easing_func)(x) * self.gain
    }

    pub fn before(&self, time: i64) -> bool {
        self.time > time
    }

    pub fn after(&self, time: i64) -> bool {
        self.time + self.duration < time
    }

    pub fn contain(&self, time: i64) -> bool {
        let t = time - self.time;
        t >= 0 && t <= self.duration
    }

    pub fn reset_gain(&mut self, gain: T) {
        self.gain = gain;
    }

    pub fn last_value(&self) -> T {
        self.gain
    }
}

pub struct TimeBaseEasingValueFactory {
    clock: fn() -> i64,
}

impl TimeBaseEasingValueFactory {
    pub fn new(clock: fn() -> i64) -> Self {
        Self { clock }
    }

    pub fn new_value<T: Float>(&self, value: T) -> TimeBaseEasingValue<T> {
        TimeBaseEasingValue {
            clock: self.clock,
            ..TimeBaseEasingValue::new(value)
        }
    }
}

pub struct TimeBaseEasingValue<T: Float> {
    value: EasingValue<T>,
    clock: fn() -> i64,
}

impl<T: Float> TimeBaseEasingValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: EasingValue::new(value),
            clock: Self::current_time,
        }
    }

    pub fn add(&mut self, gain: T, duration: Duration, easing_func: fn(T) -> T) -> bool {
        self.value.add(Gain::new(
            gain,
            (self.clock)(),
            duration.as_millis().to_i64().unwrap(),
            easing_func,
        ))
    }

    pub fn update(&mut self, gain: T, duration: Duration, easing_func: fn(T) -> T) -> bool {
        self.value.update(Gain::new(
            gain,
            (self.clock)(),
            duration.as_millis().to_i64().unwrap(),
            easing_func,
        ))
    }

    pub fn gc(&mut self) {
        self.value.gc((self.clock)());
    }

    pub fn current_value(&self) -> T {
        self.value.current_value((self.clock)())
    }

    pub fn last_value(&self) -> T {
        self.value.last_value()
    }

    pub fn in_animation(&self) -> bool {
        self.value.in_animation((self.clock)())
    }

    #[inline]
    fn current_time() -> i64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_i64()
            .unwrap()
    }
}

pub struct EasingValue<T: Float> {
    value: T,
    queue: Vec<Gain<T>>,
}

impl<T: Float> EasingValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            queue: Vec::new(),
        }
    }

    pub fn add(&mut self, gain: Gain<T>) -> bool {
        if gain.last_value() == T::zero() {
            return false;
        }
        self.queue.push(gain);
        true
    }

    pub fn update(&mut self, mut gain: Gain<T>) -> bool {
        if gain.last_value() - self.last_value() == T::zero() {
            return false;
        }
        gain.reset_gain(gain.last_value() - self.last_value());
        self.queue.push(gain);
        true
    }

    pub fn gc(&mut self, time: i64) {
        let gain: T = self
            .queue
            .iter()
            .filter(|gain| gain.after(time))
            .map(|gain| gain.calc(time))
            .fold(T::zero(), |sum, t| sum + t);

        self.value = self.value + gain;
        self.queue.retain(|gain| !gain.after(time));
    }

    pub fn current_value(&self, time: i64) -> T {
        self.queue
            .iter()
            .map(|gain| gain.calc(time))
            .fold(self.value, |sum, t| sum + t)
    }

    fn last_value(&self) -> T {
        self.queue
            .iter()
            .map(|gain| gain.last_value())
            .fold(self.value, |sum, t| sum + t)
    }

    pub fn in_animation(&self, time: i64) -> bool {
        self.queue.iter().any(|gain| !gain.after(time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easing_value_add() {
        let mut v = EasingValue::new(0.0);
        v.add(Gain::new(10.0, 1, 2, functions::liner));
        assert_eq!(v.current_value(0), 0.0);
        assert!(v.in_animation(0));

        assert_eq!(v.current_value(1), 0.0);
        assert!(v.in_animation(1));

        assert_eq!(v.current_value(2), 5.0);
        assert!(v.in_animation(2));

        assert_eq!(v.current_value(3), 10.0);
        assert!(v.in_animation(3));

        assert_eq!(v.current_value(4), 10.0);
        assert!(!v.in_animation(4));

        v.gc(4);
        assert_eq!(v.current_value(4), 10.0);
        assert!(!v.in_animation(4));
    }

    #[test]
    fn easing_value_update() {
        let mut v = EasingValue::new(5.0);
        v.update(Gain::new(10.0, 1, 2, functions::liner));
        assert_eq!(v.current_value(0), 5.0);
        assert!(v.in_animation(0));

        assert_eq!(v.current_value(1), 5.0);
        assert!(v.in_animation(1));

        assert_eq!(v.current_value(2), 7.5);
        assert!(v.in_animation(2));

        assert_eq!(v.current_value(3), 10.0);
        assert!(v.in_animation(3));

        assert_eq!(v.current_value(4), 10.0);
        assert!(!v.in_animation(4));

        v.gc(4);
        assert_eq!(v.current_value(4), 10.0);
        assert!(!v.in_animation(4));
    }

    #[test]
    fn time_base_easing_value_add() {
        let mut v = TimeBaseEasingValue::new(0.0);
        v.add(1.0, Duration::from_millis(100), functions::sin_in_out);

        loop {
            println!("value:{}", v.current_value());
            if !v.in_animation() {
                break;
            }
        }

        assert_eq!(v.current_value(), 1.0);
        v.gc();
        assert_eq!(v.current_value(), 1.0);
    }
}
