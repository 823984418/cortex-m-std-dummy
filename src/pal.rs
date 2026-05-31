use std::ptr::null_mut;

use libc::*;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn clock_gettime(clock_id: clockid_t, tp: *mut timespec) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn esp_fill_random(buf: *mut c_void, len: usize) {
    unsafe {
        static mut COUNT: u8 = 0;
        for i in std::slice::from_raw_parts_mut::<u8>(buf as *mut u8, len) {
            *i = COUNT;
            COUNT += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn getenv(s: *const c_char) -> *mut c_char {
    null_mut()
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn setenv(name: *const c_char, val: *const c_char, overwrite: c_int) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn unsetenv(name: *const c_char) -> c_int {
    0
}
