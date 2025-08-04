use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if let Ok(kernel_dir) = env::var("KERNEL_DIR") {
        println!("cargo:rerun-if-changed={}/.config", kernel_dir);
        println!(
            "cargo:warning=Kernux compiling with KERNEL_DIR={}",
            kernel_dir
        );
    } else {
        println!("cargo:warning=KERNEL_DIR environment variable not set. Build may be incomplete for kernel modules.");
    }
}