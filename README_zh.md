# cortex-m-std-dummy

此库旨在 cortex-m 系列 MCU 上使用 Rust 标准库 `std`，通过提供缺失函数实现来修复链接错误。

## 原理

以往 cortex-m 的 MCU 通常 `#![no_std]`使用Rust，为了使用 `std`， 修改对应的 json-target-spec json 并补全 `std` 所依赖的
POSIX C 库函数，从而允许链接器生成可以直接烧录的固件。

* 此库提供了这些缺失的 C 符号的简易忽略实现，函数体通常返回 0 或空指针。
* `alloc` 委托给全局分配器 分配器完成实际内存管理。
* `thumbv6m` 没有原子功能，这些操作通过 `cortex-m` 禁用中断实现。
* `thread_local` 假设为单线程，因此简单得实现为创建堆存储

通过将此库作为依赖引入，cargo 在链接阶段会使用此库提供的符号，从而满足 `std` 库的链接需求，使得在 cortex-m 裸机环境下也能链接通过。

### Feature 标志

每个功能模块对应一个独立的 feature，可按需启用：

| Feature        | 说明                  |
|----------------|---------------------|
| `alloc`        | 内存分配                |
| `atomic`       | 为 `thumbv6m` 模拟原子操作 |
| `fd`           | 文件描述符               |
| `fs`           | 文件系统                |
| `io`           | 输入输出                |
| `pal`          | 平台特定                |
| `process`      | 进程控制                |
| `sync`         | 同步原语                |
| `thread`       | 线程操作                |
| `thread_local` | 线程局部存储              |
| `unwind`       | 栈展开                 |

此外还有两个控制链接方式的 feature：

- `linkage_weak` — 使用 `#[linkage = "weak"]` 弱链接，允许用户代码覆盖此库提供的符号
- `nightly_linkage` — 启用 nightly Rust 的 `linkage` 特性

使用 `all` feature 可一次性启用所有功能。

### 使用示例

* [cortex-m-std-demo](https://github.com/823984418/cortex-m-std-demo) — 使用示例
