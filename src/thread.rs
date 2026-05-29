use std::ffi::c_int;

#[unsafe(no_mangle)]
extern "C" fn sched_yield() -> c_int {
    0
}
