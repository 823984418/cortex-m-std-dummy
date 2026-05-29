use libc::*;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_lock_test_and_set_1(ptr: *mut i8, val: i8) -> i8 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = val;
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_lock_test_and_set_2(ptr: *mut i16, val: i16) -> i16 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = val;
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_lock_test_and_set_4(ptr: *mut i32, val: i32) -> i32 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = val;
        v
    })
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_val_compare_and_swap_1(ptr: *mut i8, old: i8, new: i8) -> i8 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        if v == old {
            *ptr = new;
        }
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_val_compare_and_swap_2(ptr: *mut i16, old: i16, new: i16) -> i16 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        if v == old {
            *ptr = new;
        }
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_val_compare_and_swap_4(ptr: *mut i32, old: i32, new: i32) -> i32 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        if v == old {
            *ptr = new;
        }
        v
    })
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_add_1(ptr: *mut i8, val: i8) -> i8 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.wrapping_add(val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_add_2(ptr: *mut i16, val: i16) -> i16 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.wrapping_add(val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_add_4(ptr: *mut i32, val: i32) -> i32 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.wrapping_add(val);
        v
    })
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_sub_1(ptr: *mut i8, val: i8) -> i8 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.wrapping_sub(val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_sub_2(ptr: *mut i16, val: i16) -> i16 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.wrapping_sub(val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_sub_4(ptr: *mut i32, val: i32) -> i32 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.wrapping_sub(val);
        v
    })
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_and_1(ptr: *mut i8, val: i8) -> i8 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v & val;
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_and_2(ptr: *mut i16, val: i16) -> i16 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v & val;
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_and_4(ptr: *mut i32, val: i32) -> i32 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v & val;
        v
    })
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_nand_1(ptr: *mut i8, val: i8) -> i8 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = !(v & val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_nand_2(ptr: *mut i16, val: i16) -> i16 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = !(v & val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_nand_4(ptr: *mut i32, val: i32) -> i32 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = !(v & val);
        v
    })
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_or_1(ptr: *mut i8, val: i8) -> i8 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v | val;
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_or_2(ptr: *mut i16, val: i16) -> i16 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v | val;
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_or_4(ptr: *mut i32, val: i32) -> i32 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v | val;
        v
    })
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_xor_1(ptr: *mut i8, val: i8) -> i8 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v ^ val;
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_xor_2(ptr: *mut i16, val: i16) -> i16 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v ^ val;
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_xor_4(ptr: *mut i32, val: i32) -> i32 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v ^ val;
        v
    })
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_max_1(ptr: *mut i8, val: i8) -> i8 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.max(val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_max_2(ptr: *mut i16, val: i16) -> i16 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.max(val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_max_4(ptr: *mut i32, val: i32) -> i32 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.max(val);
        v
    })
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_min_1(ptr: *mut i8, val: i8) -> i8 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.min(val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_min_2(ptr: *mut i16, val: i16) -> i16 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.min(val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_min_4(ptr: *mut i32, val: i32) -> i32 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.min(val);
        v
    })
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_umax_1(ptr: *mut u8, val: u8) -> u8 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.max(val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_umax_2(ptr: *mut u16, val: u16) -> u16 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.max(val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_umax_4(ptr: *mut u32, val: u32) -> u32 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.max(val);
        v
    })
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_umin_1(ptr: *mut u8, val: u8) -> u8 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.min(val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_umin_2(ptr: *mut u16, val: u16) -> u16 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.min(val);
        v
    })
}
#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn __sync_fetch_and_umin_4(ptr: *mut u32, val: u32) -> u32 {
    cortex_m::interrupt::free(|_| unsafe {
        let v = *ptr;
        *ptr = v.min(val);
        v
    })
}
