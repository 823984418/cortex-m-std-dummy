use std::alloc::Layout;

use libc::*;

/// std::sys::alloc::MIN_ALIGN
const MIN_ALIGN: usize = 8;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn calloc(nobj: size_t, size: size_t) -> *mut c_void {
    unsafe {
        let Some(size) = size_t::checked_mul(nobj, size) else {
            return std::ptr::null_mut();
        };
        if size == 0 {
            return std::ptr::null_mut();
        }
        let Ok(layout) = Layout::from_size_align(size, MIN_ALIGN) else {
            return std::ptr::null_mut();
        };
        let Ok((wrap_layout, offset)) = Layout::new::<Layout>().extend(layout) else {
            return std::ptr::null_mut();
        };
        let wrap_ptr = std::alloc::alloc_zeroed(wrap_layout);
        if wrap_ptr.is_null() {
            return std::ptr::null_mut();
        }
        let ptr = wrap_ptr.add(offset);
        *(ptr.sub(size_of::<Layout>()) as *mut Layout) = layout;
        ptr as *mut c_void
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn malloc(size: size_t) -> *mut c_void {
    unsafe {
        if size == 0 {
            return std::ptr::null_mut();
        }
        let Ok(layout) = Layout::from_size_align(size, MIN_ALIGN) else {
            return std::ptr::null_mut();
        };
        let Ok((wrap_layout, offset)) = Layout::new::<Layout>().extend(layout) else {
            return std::ptr::null_mut();
        };
        let wrap_ptr = std::alloc::alloc(wrap_layout);
        if wrap_ptr.is_null() {
            return std::ptr::null_mut();
        }
        let ptr = wrap_ptr.add(offset);
        *(ptr.sub(size_of::<Layout>()) as *mut Layout) = layout;
        ptr as *mut c_void
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn realloc(p: *mut c_void, size: size_t) -> *mut c_void {
    unsafe {
        if size == 0 {
            free(p);
            return std::ptr::null_mut();
        }
        if p.is_null() {
            return malloc(size);
        }
        let ptr = p as *mut u8;
        let layout = *(ptr.sub(size_of::<Layout>()) as *const Layout);
        let Ok((wrap_layout, offset)) = Layout::new::<Layout>().extend(layout) else {
            return std::ptr::null_mut();
        };
        let wrap_ptr = ptr.sub(offset);
        let Ok(new_layout) = Layout::from_size_align(size, layout.align()) else {
            return std::ptr::null_mut();
        };
        let new_wrap_ptr = std::alloc::realloc(
            wrap_ptr,
            wrap_layout,
            wrap_layout.size() - layout.size() + new_layout.size(),
        );
        if new_wrap_ptr.is_null() {
            return std::ptr::null_mut();
        }
        let new_ptr = new_wrap_ptr.add(offset);
        *(new_ptr.sub(size_of::<Layout>()) as *mut Layout) = new_layout;
        new_ptr as *mut c_void
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn free(p: *mut c_void) {
    unsafe {
        if p.is_null() {
            return;
        }
        let ptr = p as *mut u8;
        let layout = *(ptr.sub(size_of::<Layout>()) as *const Layout);
        let Ok((wrap_layout, offset)) = Layout::new::<Layout>().extend(layout) else {
            return;
        };
        let wrap_ptr = ptr.sub(offset);
        std::alloc::dealloc(wrap_ptr, wrap_layout)
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int {
    unsafe {
        let Ok(layout) = Layout::from_size_align(size, align) else {
            return EINVAL;
        };
        if align < size_of::<*mut c_void>() {
            return EINVAL;
        }
        let Ok((wrap_layout, offset)) = Layout::new::<Layout>().extend(layout) else {
            return EINVAL;
        };
        let wrap_ptr = std::alloc::alloc(wrap_layout);
        if wrap_ptr.is_null() {
            return ENOMEM;
        }
        let ptr = wrap_ptr.add(offset);
        *(ptr.sub(size_of::<Layout>()) as *mut Layout) = layout;
        *memptr = ptr as *mut c_void;
        0
    }
}
