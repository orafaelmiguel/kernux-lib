# Kernux

[![License](https://img.shields.io/badge/license-GPL--2.0--only-blue.svg)](./LICENSE)

Kernux is a Rust crate that provides safe, ergonomic, and zero-cost abstractions over the Linux Kernel's C APIs. The goal is to enable developers to write kernel modules and drivers with the full power of Rust's safety guarantees and modern language features, reducing boilerplate and entire classes of common bugs.

## Core Principles

* **Safety First**: Encapsulate `unsafe` C API calls within safe, verifiable Rust interfaces. Let the Rust compiler be your primary line of defense.
* **Zero-Cost Abstractions**: Leverage Rust's traits, generics, and macros to create high-level abstractions that compile down to efficient, performant code with no runtime overhead.
* **Ergonomic and Idiomatic API**: Provide an API that feels natural to Rust developers. Use `Result` and `Option` for error handling, RAII for resource management (`KBox`, `KVec`), and traits for defining device behavior.
* **Declarative Macros**: Simplify complex registration and initialization boilerplate with powerful declarative macros.

### Prerequisites

To build a kernel module using Kernux, you will need a recent Rust nightly toolchain, along with the kernel headers and configured source code for your target kernel version.

### Project Structure

A typical driver built with Kernux is structured as a `staticlib` Rust crate. The `Cargo.toml` file must be configured to build without the standard library and to produce a static library archive.

### Build Integration

A `Makefile` is required to integrate with the kernel's build system (Kbuild). This `Makefile` must contain rules to first invoke `cargo build` to produce the Rust static library, and then to link that library into the final kernel module object (`.ko`) file.

### Driver Implementation

The development workflow with Kernux is designed to be declarative and focus on device logic rather than boilerplate.

1.  **State Definition**: You define a Rust struct that will hold the state for your driver.
2.  **Behavior Implementation**: You implement a specific operations trait provided by Kernux (e.g., `CharDeviceOps`, `BlockDeviceOps`) for your state struct. The methods in this trait correspond to kernel operations like `read`, `write`, or `ioctl`. All methods use safe, idiomatic Rust types.
3.  **Module Registration**: Finally, you use a declarative macro (e.g., `define_char_device!`) to register your driver. This macro automatically generates all necessary `module_init`, `module_exit`, and kernel registration functions based on the information and types you provide.

This approach abstracts away the complexities of `file_operations` structs, `cdev` management, and other low-level details, allowing you to focus on the core functionality of your driver.

## Code

The core architecture of Kernux revolves around the combination of **State Structs**, **Operations Traits**, and **Declarative Macros**.

* **State Struct**: A simple struct you define to hold any data your driver needs to maintain. This decouples the driver's state from its behavior.
* **Operations Trait** (e.g., `CharDeviceOps`): A trait that defines the callbacks for a specific device type. By implementing this trait for your state struct, you define the driver's behavior in a type-safe and idiomatic way. Kernux provides safe wrappers for kernel types like `File` and `UserSlice` to use in these methods.
* **Declarative Macro** (e.g., `define_char_device!`): This procedural macro is the glue that binds everything together. It reads your trait implementation and state struct, then generates all the low-level boilerplate that the kernel requires. This includes creating a static `file_operations` struct, populating it with wrapper functions that call your trait methods, and generating the `module_init` and `module_exit` functions to handle registration and cleanup.

The following example demonstrates these concepts by creating a simple character device at `/dev/hello_kernux`. When a user reads from this device, it returns the string "Hello from Kernux!".

```rust
#![no_std]
#![feature(const_fn_trait_bound)]

// Import the essential items from Kernux
use kernux::prelude::*;
use kernux::dev::char::{CharDevice, CharDeviceOps};
use kernux::error::KernelResult;
use kernux::fs::{File, UserSlice};

/// 1. State Definition: A struct to hold the driver's state.
/// For this simple example, it is empty.
struct HelloDevice;

/// 2. Behavior Implementation: Implement the `CharDeviceOps` trait for our state struct.
/// Kernux will handle translating these safe methods into the function pointers
/// required by the kernel's `file_operations` struct.
impl CharDeviceOps for HelloDevice {
    /// Called when a process reads from the device file (e.g., `cat /dev/hello_kernux`).
    /// The method signature uses safe, high-level types provided by Kernux.
    fn read(&self, _file: &File, mut buf: UserSlice) -> KernelResult<usize> {
        let message = b"Hello from Kernux!\n";

        // `UserSlice::copy_to_slice` is a safe wrapper around `copy_to_user`.
        // It handles memory access validation and returns a proper `KernelResult`.
        let bytes_written = buf.copy_to_slice(message)?;

        Ok(bytes_written)
    }
}

/// 3. Module Registration: Use the declarative macro to generate all boilerplate.
/// This single macro handles module metadata, initialization, cdev registration,
/// and cleanup.
kernux::define_char_device! {
    type: CharDevice<HelloDevice>,
    name: "hello_kernux",
    author: "Your Name",
    license: "GPL",
    state: HelloDevice,
}
```
## Contributing

Contributions are highly welcome! <3
