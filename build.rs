use std::path::Path;

fn main() {
    // nvcontrol uses native NVKMS ioctls for digital vibrance
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-env=NATIVE_VIBRANCE=1");

        // Check for NVIDIA drivers
        if Path::new("/dev/nvidia-modeset").exists() {
            println!("cargo:rustc-env=NVIDIA_DRIVERS_PRESENT=1");
        }
    }
}
