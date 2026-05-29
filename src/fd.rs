use libc::*;

#[unsafe(no_mangle)]
extern "C" fn close(fd: c_int) -> c_int {
    0
}

#[unsafe(no_mangle)]
extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    0
}
