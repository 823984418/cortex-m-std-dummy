# cortex-m-std-dummy

This crate aims to use the Rust standard library `std` on cortex-m series MCUs by providing missing function
implementations to fix linker errors.

## Principle

Traditionally, cortex-m MCUs use Rust with `#![no_std]`. To use `std`, the corresponding JSON target specification is
modified, and the POSIX C library functions that `std` depends on are supplemented, allowing the linker to generate
firmware that can be directly flashed.

* This crate provides simple stub implementations of these missing C symbols, with function bodies typically returning 0
  or a null pointer.
* `alloc` is delegated to the global allocator, which handles actual memory management.
* `thumbv6m` lacks atomic operations; these are implemented via disabling interrupts using `cortex-m`.
* `thread_local` assumes a single-threaded environment, thus simply implemented as heap-allocated storage.

By adding this crate as a dependency, cargo will use the symbols provided by this crate during the linking phase,
thereby satisfying the linkage requirements of the `std` library and enabling successful linking in a cortex-m
bare-metal environment.

### Feature Flags

Each functional module corresponds to an independent feature, which can be enabled as needed:

| Feature        | Description                               |
|----------------|-------------------------------------------|
| `alloc`        | Memory allocation                         |
| `atomic`       | Simulate atomic operations for `thumbv6m` |
| `fd`           | File descriptors                          |
| `fs`           | File system                               |
| `io`           | Input/Output                              |
| `pal`          | Platform abstraction layer                |
| `process`      | Process control                           |
| `sync`         | Synchronization primitives                |
| `thread`       | Thread operations                         |
| `thread_local` | Thread-local storage                      |
| `unwind`       | Stack unwinding                           |

There are also two features that control the linking method:

- `linkage_weak` — Uses `#[linkage = "weak"]` weak linking, allowing user code to override symbols provided by this
  crate
- `nightly_linkage` — Enables the `linkage` feature of nightly Rust

Use the `all` feature to enable all functionalities at once.

### Usage Example

* [cortex-m-std-demo](https://github.com/823984418/cortex-m-std-demo) — Usage example