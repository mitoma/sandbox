use std::f64::consts::PI;

use num_traits::Float;

#[inline]
fn clip<T: Float>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x| {
        if T::zero() < x {
            T::zero()
        } else if T::one() > x {
            T::one()
        } else {
            f(x)
        }
    }
}

pub trait FunctionTrait<T: Float> {
    fn easing_in(x: T) -> T;
    fn easing_out(x: T) -> T;
    fn easing_in_out(x: T) -> T;
}

#[inline]
fn easing_in<T: Float>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
    f
}

#[inline]
fn easing_out<T: Float>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x| {
        let result = f(vf::<T>(1.0) - x);
        vf::<T>(1.0) - result
    }
}

#[inline]
fn easing_in_out<T: Float>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x| {
        if vf::<T>(0.5) > x {
            f(x * vf(2.0)) / vf(2.0)
        } else {
            let x = (x - vf(0.5)) * vf(2.0);
            let x = vf::<T>(1.0) - x;
            let result = f(x);
            (vf::<T>(1.0) - result) / vf(2.0) + vf(0.5)
        }
    }
}

#[inline]
fn liner<T: Float>(x: T) -> T {
    x
}

fn l<T: Float>(x: T) -> T {
    clip(easing_in(liner))(x)
}

pub trait EasingFunction<T: Float> {
    fn value(&self, x: T) -> T {
        if T::zero() >= x {
            T::zero()
        } else if T::one() <= x {
            T::one()
        } else {
            self.inner_value(x)
        }
    }

    fn inner_value(&self, x: T) -> T;
}

#[derive(Default)]
pub struct Liner;

impl<T: Float> EasingFunction<T> for Liner {
    fn inner_value(&self, x: T) -> T {
        x
    }
}

#[derive(Default)]
pub struct Sin;

impl<T: Float> EasingFunction<T> for Sin {
    fn inner_value(&self, x: T) -> T {
        vf::<T>(1.0) - (x * vf(PI) / vf(2.0)).cos()
    }
}

#[derive(Default)]
pub struct Quad {}

impl<T: Float> EasingFunction<T> for Quad {
    fn inner_value(&self, x: T) -> T {
        x * x
    }
}

#[derive(Default)]
pub struct Cubec {}

impl<T: Float> EasingFunction<T> for Cubec {
    fn inner_value(&self, x: T) -> T {
        x * x * x
    }
}

#[derive(Default)]
pub struct Quart {}

impl<T: Float> EasingFunction<T> for Quart {
    fn inner_value(&self, x: T) -> T {
        x * x * x * x
    }
}

#[derive(Default)]
pub struct Quint {}

impl<T: Float> EasingFunction<T> for Quint {
    fn inner_value(&self, x: T) -> T {
        x * x * x * x * x
    }
}

#[derive(Default)]
pub struct Expo {}

impl<T: Float> EasingFunction<T> for Expo {
    fn inner_value(&self, x: T) -> T {
        vf::<T>(2.0).powf(vf::<T>(10.0) * x - vf::<T>(10.0))
    }
}

#[derive(Default)]
pub struct Circ {}

impl<T: Float> EasingFunction<T> for Circ {
    fn inner_value(&self, x: T) -> T {
        vf::<T>(1.0) - (vf::<T>(1.0) - x.powi(2)).sqrt()
    }
}

#[derive(Default)]
pub struct Back {}

const BACK_C1: f64 = 1.70158;
const BACK_C3: f64 = BACK_C1 + 1.0;

impl<T: Float> EasingFunction<T> for Back {
    fn inner_value(&self, x: T) -> T {
        vf::<T>(BACK_C3) * x * x * x - vf::<T>(BACK_C1) * x * x
    }
}

#[derive(Default)]
pub struct Elastic {}

const EXPO: f64 = 2.0 * PI / 3.0;

impl<T: Float> EasingFunction<T> for Elastic {
    fn inner_value(&self, x: T) -> T {
        -vf::<T>(2.0).powf(vf::<T>(10.0) * x - vf::<T>(10.0))
            * ((x * vf::<T>(10.0) - vf::<T>(10.75)) * vf::<T>(EXPO)).sin()
    }
}

#[derive(Default)]
pub struct Bounce {}

const BOUNCE_N1: f64 = 7.5625;
const BOUNCE_D1: f64 = 2.75;

impl<T: Float> EasingFunction<T> for Bounce {
    fn inner_value(&self, x: T) -> T {
        let x: T = vf::<T>(1.0) - x;
        let v1: T = vf(1.0);
        let v2: T = vf(2.0);
        let v2_5: T = vf(2.5);
        let n1: T = vf(BOUNCE_N1);
        let d1: T = vf(BOUNCE_D1);
        let result = if x < v1 / d1 {
            n1 * x * x
        } else if x < v2 / d1 {
            let x = x - (vf::<T>(1.5) / d1);
            n1 * x * x + vf(0.75)
        } else if x < v2_5 / d1 {
            let x = x - (vf::<T>(2.25) / d1);
            n1 * x * x + vf(0.9375)
        } else {
            let x = x - (vf::<T>(2.625) / d1);
            n1 * x * x + vf(0.984375)
        };
        v1 - result
    }
}

pub struct EasingIn<T>
where
    T: Float,
{
    easing_function: Box<dyn EasingFunction<T>>,
}

impl<T: Float> EasingIn<T> {
    pub fn new(easing_function: Box<dyn EasingFunction<T>>) -> Self {
        Self { easing_function }
    }
}

impl<T: Float> EasingFunction<T> for EasingIn<T> {
    fn inner_value(&self, x: T) -> T {
        self.easing_function.inner_value(x)
    }
}

pub struct EasingOut<T>
where
    T: Float,
{
    easing_function: Box<dyn EasingFunction<T>>,
}

impl<T: Float> EasingOut<T> {
    pub fn new(easing_function: Box<dyn EasingFunction<T>>) -> Self {
        Self { easing_function }
    }
}

impl<T: Float> EasingFunction<T> for EasingOut<T> {
    fn inner_value(&self, x: T) -> T {
        let result = self.easing_function.inner_value(vf::<T>(1.0) - x);
        vf::<T>(1.0) - result
    }
}

pub struct InOut<T>
where
    T: Float,
{
    in_easing_function: Box<dyn EasingFunction<T>>,
    out_easing_function: Box<dyn EasingFunction<T>>,
}

impl<T: Float> InOut<T> {
    pub fn new(
        in_easing_function: Box<dyn EasingFunction<T>>,
        out_easing_function: Box<dyn EasingFunction<T>>,
    ) -> Self {
        Self {
            in_easing_function,
            out_easing_function,
        }
    }
}

impl<T: Float> EasingFunction<T> for InOut<T> {
    fn inner_value(&self, x: T) -> T {
        if vf::<T>(0.5) > x {
            self.in_easing_function.inner_value(x * vf(2.0)) / vf(2.0)
        } else {
            let x = (x - vf(0.5)) * vf(2.0);
            let x = vf::<T>(1.0) - x;
            let result = self.out_easing_function.inner_value(x);
            (vf::<T>(1.0) - result) / vf(2.0) + vf(0.5)
        }
    }
}

#[inline]
fn vf<T: Float>(x: f64) -> T {
    T::from(x).unwrap()
}
