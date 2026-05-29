use libc::*;

#[unsafe(export_name = "__errno")]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn errno_location() -> *mut c_int {
    static mut ERRNO: c_int = 0;
    unsafe { &raw mut ERRNO }
}

#[unsafe(export_name = "__xpg_strerror_r")]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int {
    0
}
