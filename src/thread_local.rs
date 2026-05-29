use std::alloc::Layout;
use std::ptr::null_mut;

use libc::*;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pthread_key_create(
    key: *mut pthread_key_t,
    dtor: Option<unsafe extern "C" fn(*mut c_void)>,
) -> c_int {
    unsafe {
        let ptr = std::alloc::alloc(Layout::new::<(
            *mut c_void,
            Option<unsafe extern "C" fn(*mut c_void)>,
        )>()) as *mut (*mut c_void, Option<unsafe extern "C" fn(*mut c_void)>);
        if ptr.is_null() {
            return 1;
        }
        (*ptr).0 = null_mut();
        (*ptr).1 = dtor;
        *key = ptr as pthread_key_t;
        0
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pthread_key_delete(key: pthread_key_t) -> c_int {
    unsafe {
        let ptr = key as *mut (*mut c_void, Option<unsafe extern "C" fn(*mut c_void)>);
        if ptr.is_null() {
            return 1;
        }
        loop {
            let value = (*ptr).0;
            (*ptr).0 = null_mut();
            if value.is_null() {
                break;
            } else if let Some(dtor) = (*ptr).1 {
                dtor(value);
            } else {
                break;
            }
        }
        std::alloc::dealloc(
            ptr as *mut u8,
            Layout::new::<(*mut c_void, Option<unsafe extern "C" fn(*mut c_void)>)>(),
        );
        0
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pthread_getspecific(key: pthread_key_t) -> *mut c_void {
    unsafe {
        let ptr = key as *mut (*mut c_void, Option<unsafe extern "C" fn(*mut c_void)>);
        (*ptr).0
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn pthread_setspecific(key: pthread_key_t, value: *const c_void) -> c_int {
    unsafe {
        let ptr = key as *mut (*mut c_void, Option<unsafe extern "C" fn(*mut c_void)>);
        if ptr.is_null() {
            return 1;
        }
        (*ptr).0 = value as *mut c_void;
        0
    }
}
