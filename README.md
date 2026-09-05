# cortex-m-std-dummy

This crate aims to use the Rust standard library `std` on MCUs by providing missing function implementations to fix
linker errors.

Although this crate was originally designed for Cortex-M targets, it has since been extended to other architectures.
Currently provided targets include:

* riscv32i-unknown-none-elf
* riscv32im-unknown-none-elf
* riscv32ima-unknown-none-elf
* riscv32imc-unknown-none-elf
* riscv32imfc-unknown-none-elf
* riscv32imac-unknown-none-elf
* riscv32imafc-unknown-none-elf
* thumbv6m-none-eabi
* thumbv7em-none-eabi
* thumbv7em-none-eabihf
* thumbv7m-none-eabi
* thumbv8m.base-none-eabi
* thumbv8m.main-none-eabi
* thumbv8m.main-none-eabihf

This crate borrows `espidf` to provide standard library functionality. As a tier-3 target, Rust does not guarantee that
the standard library will always compile; the last known Rust version that compiles successfully is
`nightly-2026-07-28`

## Principle

Traditionally, MCUs use Rust with `#![no_std]`. To use `std`, the corresponding JSON target specification is modified,
and the POSIX C library functions that `std` depends on are supplemented, allowing the linker to generate firmware that
can be directly flashed.

* This crate provides simple stub implementations of these missing C symbols, with function bodies typically returning 0
  or a null pointer.
* `alloc` is delegated to the global allocator, which handles actual memory management.
* Some targets lack atomic operations; these are simulated via `critical-section`.
* `thread_local` assumes a single-threaded environment, thus simply implemented as heap-allocated storage.

By adding this crate as a dependency, cargo will use the symbols provided by this crate during the linking phase,
thereby satisfying the linkage requirements of the `std` library and enabling successful linking in a cortex-m
bare-metal environment.

### Feature Flags

Each functional module corresponds to an independent feature, which can be enabled as needed:

| Feature        | Description                                       |
|----------------|---------------------------------------------------|
| `alloc`        | Memory allocation                                 |
| `atomic`       | Simulate atomic operations via `critical-section` |
| `fd`           | File descriptors                                  |
| `fs`           | File system                                       |
| `io`           | Input/Output                                      |
| `pal`          | Platform specific                                 |
| `process`      | Process control                                   |
| `sync`         | Synchronization primitives                        |
| `thread`       | Thread operations                                 |
| `thread_local` | Thread-local storage                              |
| `unwind`       | Stack unwinding                                   |
| `math`         | Math                                              |

There are also two features that control the linking method:

- `linkage_weak` — Uses `#[linkage = "weak"]` weak linking, allowing user code to override symbols provided by this
  crate
- `nightly_linkage` — Enables the `linkage` feature of nightly Rust

Use the `all` feature to enable all functionalities at once.

### Usage Example

* [cortex-m-std-demo](https://github.com/823984418/cortex-m-std-demo) — Usage example
