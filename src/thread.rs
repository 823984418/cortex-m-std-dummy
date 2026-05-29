use libc::*;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pthread_create(
    native: *mut pthread_t,
    attr: *const pthread_attr_t,
    f: extern "C" fn(*mut c_void) -> *mut c_void,
    value: *mut c_void,
) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pthread_attr_setstacksize(attr: *mut pthread_attr_t, stack_size: size_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pthread_join(native: pthread_t, value: *mut *mut c_void) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pthread_detach(thread: pthread_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn sched_yield() -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn usleep(secs: c_uint) -> c_int {
    0
}
