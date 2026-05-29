use libc::*;

#[unsafe(no_mangle)]
extern "C" fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
extern "C" fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
extern "C" fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, _type: c_int) -> c_int {
    0
}

#[unsafe(no_mangle)]
extern "C" fn pthread_mutex_init(
    lock: *mut pthread_mutex_t,
    attr: *const pthread_mutexattr_t,
) -> c_int {
    0
}

#[unsafe(no_mangle)]
extern "C" fn pthread_mutex_destroy(lock: *mut pthread_mutex_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
extern "C" fn pthread_mutex_lock(lock: *mut pthread_mutex_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
extern "C" fn pthread_mutex_trylock(lock: *mut pthread_mutex_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
extern "C" fn pthread_mutex_unlock(lock: *mut pthread_mutex_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
extern "C" fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
extern "C" fn pthread_cond_wait(cond: *mut pthread_cond_t, lock: *mut pthread_mutex_t) -> c_int {
    0
}

#[unsafe(no_mangle)]
extern "C" fn pthread_cond_init(
    cond: *mut pthread_cond_t,
    attr: *const pthread_condattr_t,
) -> c_int {
    0
}

#[unsafe(no_mangle)]
extern "C" fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> c_int {
    0
}
