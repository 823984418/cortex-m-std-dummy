#[unsafe(no_mangle)]
extern "C" fn abort() -> ! {
    loop {}
}
