use libc::*;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn open(path: *const c_char, oflag: c_int /*, ...*/) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
unsafe extern "C" fn fcntl(fd: c_int, cmd: c_int /*, ...*/) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pipe(fds: *mut c_int) -> c_int {
    0
}
