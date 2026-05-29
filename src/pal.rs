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
        for i in std::slice::from_raw_parts_mut::<u8>(buf as *mut u8, len) {
            let v = cortex_m::peripheral::SYST::get_current().to_le_bytes();
            *i = v[0] ^ v[1] ^ v[2] ^ v[3];
        }
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn getenv(s: *const c_char) -> *mut c_char {
    null_mut()
}
