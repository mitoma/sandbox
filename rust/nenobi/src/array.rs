use num_traits::Float;

pub struct Gain<T: Float, const N: usize> {
    gain: [T; N],
    time: i64,
    duration: i64,
    easing_func: fn(T) -> T,
}

impl<T: Float, const N: usize> Gain<T, N> {
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

pub struct EasingValue<T: Float, const N: usize> {
    value: [T; N],
    queue: Vec<Gain<T, N>>,
}

impl<T: Float, const N: usize> EasingValue<T, N> {
    pub fn new(value: [T; N]) -> Self {
        Self {
            value,
            queue: Vec::new(),
        }
    }

    pub fn add(&mut self, gain: Gain<T, N>) -> bool {
        if gain.last_value() == [T::zero(); N] {
            return false;
        }
        self.queue.push(gain);
        true
    }

    pub fn update(&mut self, mut gain: Gain<T, N>) -> bool {
        let sub = [T::zero(); N];

        let gain_last_value = gain.last_value();
        let self_last_value = self.last_value();
        let all_zero = true;
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

    fn last_value(&self) -> [T; N] {
        self.queue
            .iter()
            .map(|gain| gain.last_value())
            .fold(self.value, |sum, t| sum + t)
    }

    pub fn in_animation(&self, time: i64) -> bool {
        self.queue.iter().any(|gain| !gain.after(time))
    }
}
