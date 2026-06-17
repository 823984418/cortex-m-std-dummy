use std::ffi::{c_int, c_void};

use libc::{size_t, ssize_t};
use libm::Libm;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn powf(x: f32, y: f32) -> f32 {
    Libm::<f32>::pow(x, y)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn expf(x: f32) -> f32 {
    Libm::<f32>::exp(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn exp2f(x: f32) -> f32 {
    Libm::<f32>::exp2(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn logf(x: f32) -> f32 {
    Libm::<f32>::log(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log2f(x: f32) -> f32 {
    Libm::<f32>::log2(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log10f(x: f32) -> f32 {
    Libm::<f32>::log10(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn hypotf(x: f32, y: f32) -> f32 {
    Libm::<f32>::hypot(x, y)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn sinf(x: f32) -> f32 {
    Libm::<f32>::sin(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn cosf(x: f32) -> f32 {
    Libm::<f32>::cos(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn tanf(x: f32) -> f32 {
    Libm::<f32>::tan(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn asinf(x: f32) -> f32 {
    Libm::<f32>::asin(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn acosf(x: f32) -> f32 {
    Libm::<f32>::acos(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn atanf(x: f32) -> f32 {
    Libm::<f32>::atan(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn atan2f(y: f32, x: f32) -> f32 {
    Libm::<f32>::atan2(y, x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn expm1f(x: f32) -> f32 {
    Libm::<f32>::expm1(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log1pf(x: f32) -> f32 {
    Libm::<f32>::log1p(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn sinhf(x: f32) -> f32 {
    Libm::<f32>::sinh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn coshf(x: f32) -> f32 {
    Libm::<f32>::cosh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn tanhf(x: f32) -> f32 {
    Libm::<f32>::tanh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn asinhf(x: f32) -> f32 {
    Libm::<f32>::asinh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn acoshf(x: f32) -> f32 {
    Libm::<f32>::acosh(x)
}


#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn tgammaf(x: f32) -> f32 {
    Libm::<f32>::tgamma(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn lgammaf_r(n: f32, s: &mut i32) -> f32 {
    let (x, y) = Libm::<f32>::lgamma_r(n);
    *s = y;
    x
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn erff(x: f32) -> f32 {
    Libm::<f32>::erf(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn erfcf(x: f32) -> f32 {
    Libm::<f32>::erfc(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pow(x: f64, y: f64) -> f64 {
    Libm::<f64>::pow(x, y)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn exp(x: f64) -> f64 {
    Libm::<f64>::exp(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn exp2(x: f64) -> f64 {
    Libm::<f64>::exp2(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log(x: f64) -> f64 {
    Libm::<f64>::log(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log2(x: f64) -> f64 {
    Libm::<f64>::log2(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log10(x: f64) -> f64 {
    Libm::<f64>::log10(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn hypot(x: f64, y: f64) -> f64 {
    Libm::<f64>::hypot(x, y)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn sin(x: f64) -> f64 {
    Libm::<f64>::sin(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn cos(x: f64) -> f64 {
    Libm::<f64>::cos(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn tan(x: f64) -> f64 {
    Libm::<f64>::tan(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn asin(x: f64) -> f64 {
    Libm::<f64>::asin(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn acos(x: f64) -> f64 {
    Libm::<f64>::acos(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn atan(x: f64) -> f64 {
    Libm::<f64>::atan(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn atan2(y: f64, x: f64) -> f64 {
    Libm::<f64>::atan2(y, x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn expm1(x: f64) -> f64 {
    Libm::<f64>::expm1(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log1p(x: f64) -> f64 {
    Libm::<f64>::log1p(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn sinh(x: f64) -> f64 {
    Libm::<f64>::sinh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn cosh(x: f64) -> f64 {
    Libm::<f64>::cosh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn tanh(x: f64) -> f64 {
    Libm::<f64>::tanh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn asinh(x: f64) -> f64 {
    Libm::<f64>::asinh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn acosh(x: f64) -> f64 {
    Libm::<f64>::acosh(x)
}


#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn tgamma(x: f64) -> f64 {
    Libm::<f64>::tgamma(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn lgamma_r(n: f64, s: &mut i32) -> f64 {
    let (x, y) = Libm::<f64>::lgamma_r(n);
    *s = y;
    x
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn erf(x: f64) -> f64 {
    Libm::<f64>::erf(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn erfc(x: f64) -> f64 {
    Libm::<f64>::erfc(x)
}
