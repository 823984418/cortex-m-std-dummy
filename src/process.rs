use libc::*;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn abort() -> ! {
    cortex_m::asm::bkpt();
    cortex_m::asm::udf();
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn exit(status: c_int) -> ! {
    cortex_m::asm::bkpt();
    cortex_m::asm::udf();
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn getpid() -> pid_t {
    0
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "linkage_weak", linkage = "weak")]
extern "C" fn getppid() -> pid_t {
    0
}
