use libc::*;
use libm::Libm;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn powf(x: c_float, y: c_float) -> c_float {
    Libm::<c_float>::pow(x, y)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn expf(x: c_float) -> c_float {
    Libm::<c_float>::exp(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn exp2f(x: c_float) -> c_float {
    Libm::<c_float>::exp2(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn logf(x: c_float) -> c_float {
    Libm::<c_float>::log(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log2f(x: c_float) -> c_float {
    Libm::<c_float>::log2(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log10f(x: c_float) -> c_float {
    Libm::<c_float>::log10(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn hypotf(x: c_float, y: c_float) -> c_float {
    Libm::<c_float>::hypot(x, y)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn sinf(x: c_float) -> c_float {
    Libm::<c_float>::sin(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn cosf(x: c_float) -> c_float {
    Libm::<c_float>::cos(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn tanf(x: c_float) -> c_float {
    Libm::<c_float>::tan(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn asinf(x: c_float) -> c_float {
    Libm::<c_float>::asin(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn acosf(x: c_float) -> c_float {
    Libm::<c_float>::acos(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn atanf(x: c_float) -> c_float {
    Libm::<c_float>::atan(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn atan2f(y: c_float, x: c_float) -> c_float {
    Libm::<c_float>::atan2(y, x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn expm1f(x: c_float) -> c_float {
    Libm::<c_float>::expm1(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log1pf(x: c_float) -> c_float {
    Libm::<c_float>::log1p(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn sinhf(x: c_float) -> c_float {
    Libm::<c_float>::sinh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn coshf(x: c_float) -> c_float {
    Libm::<c_float>::cosh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn tanhf(x: c_float) -> c_float {
    Libm::<c_float>::tanh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn asinhf(x: c_float) -> c_float {
    Libm::<c_float>::asinh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn acoshf(x: c_float) -> c_float {
    Libm::<c_float>::acosh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn tgammaf(x: c_float) -> c_float {
    Libm::<c_float>::tgamma(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn lgammaf_r(n: c_float, s: *mut c_int) -> c_float {
    let (x, y) = Libm::<c_float>::lgamma_r(n);
    unsafe {
        *s = y;
    }
    x
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn erff(x: c_float) -> c_float {
    Libm::<c_float>::erf(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn erfcf(x: c_float) -> c_float {
    Libm::<c_float>::erfc(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pow(x: c_double, y: c_double) -> c_double {
    Libm::<c_double>::pow(x, y)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn exp(x: c_double) -> c_double {
    Libm::<c_double>::exp(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn exp2(x: c_double) -> c_double {
    Libm::<c_double>::exp2(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log(x: c_double) -> c_double {
    Libm::<c_double>::log(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log2(x: c_double) -> c_double {
    Libm::<c_double>::log2(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log10(x: c_double) -> c_double {
    Libm::<c_double>::log10(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn hypot(x: c_double, y: c_double) -> c_double {
    Libm::<c_double>::hypot(x, y)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn sin(x: c_double) -> c_double {
    Libm::<c_double>::sin(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn cos(x: c_double) -> c_double {
    Libm::<c_double>::cos(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn tan(x: c_double) -> c_double {
    Libm::<c_double>::tan(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn asin(x: c_double) -> c_double {
    Libm::<c_double>::asin(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn acos(x: c_double) -> c_double {
    Libm::<c_double>::acos(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn atan(x: c_double) -> c_double {
    Libm::<c_double>::atan(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn atan2(y: c_double, x: c_double) -> c_double {
    Libm::<c_double>::atan2(y, x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn expm1(x: c_double) -> c_double {
    Libm::<c_double>::expm1(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn log1p(x: c_double) -> c_double {
    Libm::<c_double>::log1p(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn sinh(x: c_double) -> c_double {
    Libm::<c_double>::sinh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn cosh(x: c_double) -> c_double {
    Libm::<c_double>::cosh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn tanh(x: c_double) -> c_double {
    Libm::<c_double>::tanh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn asinh(x: c_double) -> c_double {
    Libm::<c_double>::asinh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn acosh(x: c_double) -> c_double {
    Libm::<c_double>::acosh(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn tgamma(x: c_double) -> c_double {
    Libm::<c_double>::tgamma(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn lgamma_r(n: c_double, s: *mut c_int) -> c_double {
    let (x, y) = Libm::<c_double>::lgamma_r(n);
    unsafe {
        *s = y;
    }
    x
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn erf(x: c_double) -> c_double {
    Libm::<c_double>::erf(x)
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn erfc(x: c_double) -> c_double {
    Libm::<c_double>::erfc(x)
}
