use crate::function_macro::{create_easing_in, create_easing_in_out, create_easing_out};
use num_traits::Float;
use std::f64::consts::PI;

#[inline]
fn clip<T: Float>(x: T, f: impl Fn(T) -> T) -> T {
    if T::zero() > x {
        T::zero()
    } else if T::one() < x {
        T::one()
    } else {
        f(x)
    }
}

#[inline]
pub fn liner<T: Float>(x: T) -> T {
    x
}

#[inline]
fn sin_internal<T: Float>(x: T) -> T {
    vf::<T>(1.0) - (x * vf(PI) / vf(2.0)).cos()
}

create_easing_in!(sin_in, sin_internal);
create_easing_out!(sin_out, sin_internal);
create_easing_in_out!(sin_in_out, sin_internal);

#[inline]
fn quad_internal<T: Float>(x: T) -> T {
    x * x
}

create_easing_in!(quad_in, quad_internal);
create_easing_out!(quad_out, quad_internal);
create_easing_in_out!(quad_in_out, quad_internal);

#[inline]
fn cubic_internal<T: Float>(x: T) -> T {
    x * x * x
}

create_easing_in!(cubic_in, cubic_internal);
create_easing_out!(cubic_out, cubic_internal);
create_easing_in_out!(cubic_in_out, cubic_internal);

#[inline]
fn quart_internal<T: Float>(x: T) -> T {
    x * x * x * x
}

create_easing_in!(quart_in, quart_internal);
create_easing_out!(quart_out, quart_internal);
create_easing_in_out!(quart_in_out, quart_internal);

#[inline]
fn quint_internal<T: Float>(x: T) -> T {
    x * x * x * x * x
}

create_easing_in!(quint_in, quint_internal);
create_easing_out!(quint_out, quint_internal);
create_easing_in_out!(quint_in_out, quint_internal);

#[inline]
fn expo_internal<T: Float>(x: T) -> T {
    vf::<T>(2.0).powf(vf::<T>(10.0) * x - vf::<T>(10.0))
}

create_easing_in!(expo_in, expo_internal);
create_easing_out!(expo_out, expo_internal);
create_easing_in_out!(expo_in_out, expo_internal);

#[inline]
fn circ_internal<T: Float>(x: T) -> T {
    vf::<T>(1.0) - (vf::<T>(1.0) - x.powi(2)).sqrt()
}

create_easing_in!(circ_in, circ_internal);
create_easing_out!(circ_out, circ_internal);
create_easing_in_out!(circ_in_out, circ_internal);

const BACK_C1: f64 = 1.70158;
const BACK_C3: f64 = BACK_C1 + 1.0;

#[inline]
fn back_internal<T: Float>(x: T) -> T {
    vf::<T>(BACK_C3) * x * x * x - vf::<T>(BACK_C1) * x * x
}

create_easing_in!(back_in, back_internal);
create_easing_out!(back_out, back_internal);
create_easing_in_out!(back_in_out, back_internal);

const EXPO: f64 = 2.0 * PI / 3.0;

#[inline]
fn elastic_internal<T: Float>(x: T) -> T {
    -vf::<T>(2.0).powf(vf::<T>(10.0) * x - vf::<T>(10.0))
        * ((x * vf::<T>(10.0) - vf::<T>(10.75)) * vf::<T>(EXPO)).sin()
}

create_easing_in!(elastic_in, elastic_internal);
create_easing_out!(elastic_out, elastic_internal);
create_easing_in_out!(elastic_in_out, elastic_internal);

const BOUNCE_N1: f64 = 7.5625;
const BOUNCE_D1: f64 = 2.75;

#[inline]
fn bounce_internal<T: Float>(x: T) -> T {
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

create_easing_in!(bounce_in, bounce_internal);
create_easing_out!(bounce_out, bounce_internal);
create_easing_in_out!(bounce_in_out, bounce_internal);

#[inline]
fn vf<T: Float>(x: f64) -> T {
    T::from(x).unwrap()
}
