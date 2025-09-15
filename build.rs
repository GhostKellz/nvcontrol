use std::path::Path;

fn main() {
    // nvcontrol now uses native Rust vibrance implementation
    // No Python nvibrant dependency needed
    if cfg!(target_os = "linux") {
        println!("cargo:warning=Using native Rust vibrance implementation");
        println!("cargo:rustc-env=NATIVE_VIBRANCE=1");

        // Check for NVIDIA drivers
        if Path::new("/dev/nvidia-modeset").exists() {
            println!("cargo:rustc-env=NVIDIA_DRIVERS_PRESENT=1");
            println!("cargo:warning=✅ NVIDIA drivers detected - native vibrance available");
        } else {
            println!("cargo:warning=⚠️ NVIDIA drivers not detected - vibrance may not work");
        }
    }
}
