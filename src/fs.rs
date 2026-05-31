use std::ptr::null_mut;

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

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn opendir(dirname: *const c_char) -> *mut DIR {
    null_mut()
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn closedir(dirp: *mut DIR) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn lstat(path: *const c_char, buf: *mut stat) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn mkdir(path: *const c_char, mode: mode_t) -> c_int {
    0
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn stat(path: *const c_char, buf: *mut stat) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn rmdir(path: *const c_char) -> c_int {
    0
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn unlink(c: *const c_char) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn readdir_r(dirp: *mut DIR, entry: *mut dirent, result: *mut *mut dirent) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn realpath(pathname: *const c_char, resolved: *mut c_char) -> *mut c_char {
    null_mut()
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn link(src: *const c_char, dst: *const c_char) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn symlink(path1: *const c_char, path2: *const c_char) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn chmod(path: *const c_char, mode: mode_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn rename(oldname: *const c_char, newname: *const c_char) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn fsync(fd: c_int) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn isatty(fd: c_int) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn ftruncate(fd: c_int, length: off_t) -> c_int {
    0
}
