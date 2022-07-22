macro_rules! create_easing_in {
    ($func_name:ident, $internal_func_name:ident) => {
        #[inline]
        pub fn $func_name<T: Float>(x: T) -> T {
            clip(x, |x| $internal_func_name(x))
        }
    };
}

macro_rules! create_easing_out {
    ($func_name:ident, $internal_func_name:ident) => {
        #[inline]
        pub fn $func_name<T: Float>(x: T) -> T {
            clip(x, |x| {
                let result = $internal_func_name(vf::<T>(1.0) - x);
                vf::<T>(1.0) - result
            })
        }
    };
}

macro_rules! create_easing_in_out {
    ($func_name:ident, $internal_func_name:ident) => {
        create_easing_in_out!($func_name, $internal_func_name, $internal_func_name);
    };
    ($func_name:ident, $internal_in_func_name:ident, $internal_out_func_name:ident) => {
        #[inline]
        pub fn $func_name<T: Float>(x: T) -> T {
            clip(x, |x| {
                if vf::<T>(0.5) > x {
                    $internal_in_func_name(x * vf(2.0)) / vf(2.0)
                } else {
                    let x = (x - vf(0.5)) * vf(2.0);
                    let x = vf::<T>(1.0) - x;
                    let result = $internal_out_func_name(x);
                    (vf::<T>(1.0) - result) / vf(2.0) + vf(0.5)
                }
            })
        }
    };
}

pub(crate) use create_easing_in;
pub(crate) use create_easing_in_out;
pub(crate) use create_easing_out;
