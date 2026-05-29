use std::alloc::Layout;

use libc::*;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn calloc(nobj: size_t, size: size_t) -> *mut c_void {
    unsafe {
        let size = nobj * size;
        let layout = Layout::from_size_align_unchecked(size, 4);
        let (layout, offset) = Layout::new::<Layout>().extend(layout).unwrap_unchecked();
        let ptr = std::alloc::alloc_zeroed(layout);
        if ptr.is_null() {
            return ptr as *mut c_void;
        }
        *(ptr.add(offset).sub(layout.align().max(size_of::<Layout>())) as *mut Layout) = layout;
        ptr.add(offset) as *mut c_void
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn malloc(size: size_t) -> *mut c_void {
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, 4);
        let (layout, offset) = Layout::new::<Layout>().extend(layout).unwrap_unchecked();
        let ptr = std::alloc::alloc(layout);
        if ptr.is_null() {
            return ptr as *mut c_void;
        }
        *(ptr.add(offset).sub(layout.align().max(size_of::<Layout>())) as *mut Layout) = layout;
        ptr.add(offset) as *mut c_void
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn realloc(p: *mut c_void, size: size_t) -> *mut c_void {
    unsafe {
        let ptr = p as *mut u8;
        let layout = *(ptr.sub(size_of::<Layout>()) as *const Layout);
        let ptr = ptr.sub(layout.align().max(size_of::<Layout>()));
        std::alloc::realloc(ptr, layout, size) as *mut c_void
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn free(p: *mut c_void) {
    unsafe {
        let ptr = p as *mut u8;
        let layout = *(ptr.sub(size_of::<Layout>()) as *const Layout);
        let ptr = ptr.sub(layout.align().max(size_of::<Layout>()));
        std::alloc::dealloc(ptr, layout)
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int {
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, align);
        let (layout, offset) = Layout::new::<Layout>().extend(layout).unwrap_unchecked();
        let ptr = std::alloc::alloc(layout);
        if ptr.is_null() {
            return 1;
        }
        *(ptr.add(offset).sub(layout.align().max(size_of::<Layout>())) as *mut Layout) = layout;
        *memptr = ptr.add(offset) as *mut c_void;
        0
    }
}
