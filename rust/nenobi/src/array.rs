use instant::{Duration, SystemTime};
use num_traits::{Float, ToPrimitive};

pub struct GainN<T: Float, const N: usize> {
    gain: [T; N],
    time: i64,
    duration: i64,
    easing_func: fn(T) -> T,
}

impl<T: Float, const N: usize> GainN<T, N> {
    pub fn new(gain: [T; N], time: i64, duration: i64, easing_func: fn(T) -> T) -> Self {
        Self {
            gain,
            time,
            duration,
            easing_func,
        }
    }

    pub fn calc(&self, time: i64) -> [T; N] {
        let x = T::from(time - self.time).unwrap() / T::from(self.duration).unwrap();
        let mut result = [T::zero(); N];
        for (i, g) in self.gain.iter().enumerate() {
            result[i] = (self.easing_func)(x) * *g;
        }
        result
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

    pub fn reset_gain(&mut self, gain: [T; N]) {
        self.gain = gain;
    }

    pub fn last_value(&self) -> [T; N] {
        self.gain
    }
}

pub struct TimeBaseEasingValueN<T: Float, const N: usize>(EasingValueN<T, N>);

impl<T: Float, const N: usize> TimeBaseEasingValueN<T, N> {
    pub fn new(value: [T; N]) -> Self {
        Self(EasingValueN::new(value))
    }

    pub fn add(&mut self, gain: [T; N], duration: Duration, easing_func: fn(T) -> T) -> bool {
        self.0.add(GainN::new(
            gain,
            self.current_time(),
            duration.as_millis().to_i64().unwrap(),
            easing_func,
        ))
    }

    pub fn update(&mut self, gain: [T; N], duration: Duration, easing_func: fn(T) -> T) -> bool {
        self.0.update(GainN::new(
            gain,
            self.current_time(),
            duration.as_millis().to_i64().unwrap(),
            easing_func,
        ))
    }

    pub fn gc(&mut self) {
        self.0.gc(self.current_time());
    }

    pub fn current_value(&self) -> [T; N] {
        self.0.current_value(self.current_time())
    }

    pub fn last_value(&self) -> [T; N] {
        self.0.last_value()
    }

    pub fn in_animation(&self) -> bool {
        self.0.in_animation(self.current_time())
    }

    #[inline]
    fn current_time(&self) -> i64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_i64()
            .unwrap()
    }
}

pub struct EasingValueN<T: Float, const N: usize> {
    value: [T; N],
    queue: Vec<GainN<T, N>>,
}

impl<T: Float, const N: usize> EasingValueN<T, N> {
    pub fn new(value: [T; N]) -> Self {
        Self {
            value,
            queue: Vec::new(),
        }
    }

    pub fn add(&mut self, gain: GainN<T, N>) -> bool {
        if gain.last_value() == [T::zero(); N] {
            return false;
        }
        self.queue.push(gain);
        true
    }

    pub fn update(&mut self, mut gain: GainN<T, N>) -> bool {
        let gain_last_value = gain.last_value();
        let self_last_value = self.last_value();

        let mut sub = [T::zero(); N];
        let mut all_zero = true;
        for i in 0..N {
            sub[i] = gain_last_value[i] - self_last_value[i];
            if sub[i] != T::zero() {
                all_zero = false;
            }
        }

        if all_zero {
            return false;
        }

        gain.reset_gain(sub);
        self.queue.push(gain);
        true
    }

    pub fn gc(&mut self, time: i64) {
        let gain: [T; N] = self
            .queue
            .iter()
            .filter(|gain| gain.after(time))
            .map(|gain| gain.calc(time))
            .fold([T::zero(); N], |sum, t| {
                let mut result = [T::zero(); N];
                for i in 0..N {
                    result[i] = sum[i] + t[i];
                }
                result
            });
        let mut sum = [T::zero(); N];
        for i in 0..N {
            sum[i] = self.value[i] + gain[i];
        }
        self.value = sum;
        self.queue.retain(|gain| !gain.after(time));
    }

    pub fn current_value(&self, time: i64) -> [T; N] {
        self.queue
            .iter()
            .map(|gain| gain.calc(time))
            .fold(self.value, |sum, t| {
                let mut result = [T::zero(); N];
                for i in 0..N {
                    result[i] = sum[i] + t[i];
                }
                result
            })
    }

    fn last_value(&self) -> [T; N] {
        self.queue
            .iter()
            .map(|gain| gain.last_value())
            .fold(self.value, |sum, t| {
                let mut result = [T::zero(); N];
                for i in 0..N {
                    result[i] = sum[i] + t[i];
                }
                result
            })
    }

    pub fn in_animation(&self, time: i64) -> bool {
        self.queue.iter().any(|gain| !gain.after(time))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        array::{EasingValueN, GainN},
        functions,
    };

    #[test]
    fn easing_value_add() {
        let mut v = EasingValueN::new([0.0, 1.0]);
        v.add(GainN::new([10.0, 1.0], 1, 2, functions::liner));
        assert_eq!(v.current_value(0), [0.0, 1.0]);
        assert!(v.in_animation(0));

        assert_eq!(v.current_value(1), [0.0, 1.0]);
        assert!(v.in_animation(1));

        assert_eq!(v.current_value(2), [5.0, 1.5]);
        assert!(v.in_animation(2));

        assert_eq!(v.current_value(3), [10.0, 2.0]);
        assert!(v.in_animation(3));

        assert_eq!(v.current_value(4), [10.0, 2.0]);
        assert!(!v.in_animation(4));

        v.gc(4);
        assert_eq!(v.current_value(4), [10.0, 2.0]);
        assert!(!v.in_animation(4));
    }

    #[test]
    fn easing_value_update() {
        let mut v = EasingValueN::new([5.0, 100.0]);
        v.update(GainN::new([10.0, 50.0], 1, 2, functions::liner));
        assert_eq!(v.current_value(0), [5.0, 100.0]);
        assert!(v.in_animation(0));

        assert_eq!(v.current_value(1), [5.0, 100.0]);
        assert!(v.in_animation(1));

        assert_eq!(v.current_value(2), [7.5, 75.0]);
        assert!(v.in_animation(2));

        assert_eq!(v.current_value(3), [10.0, 50.0]);
        assert!(v.in_animation(3));

        assert_eq!(v.current_value(4), [10.0, 50.0]);
        assert!(!v.in_animation(4));

        v.gc(4);
        assert_eq!(v.current_value(4), [10.0, 50.0]);
        assert!(!v.in_animation(4));
    }
}
