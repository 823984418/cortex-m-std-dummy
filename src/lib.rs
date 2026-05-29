#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

#[cfg(feature = "alloc")]
mod alloc;

#[cfg(feature = "atomic")]
mod atomic;

#[cfg(feature = "fd")]
mod fd;

#[cfg(feature = "io")]
mod io;

#[cfg(feature = "process")]
mod process;

#[cfg(feature = "sync")]
mod sync;

#[cfg(feature = "thread")]
mod thread;

#[cfg(feature = "thread_local")]
mod thread_local;

#[cfg(feature = "unwind")]
mod unwind;
