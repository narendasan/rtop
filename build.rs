use std::env;
use std::path::Path;
use std::os::unix::fs;

fn main() -> std::io::Result<()> {
    let gpu_monitor = env::var_os("CARGO_FEATURE_GPU-MONITOR").is_some();
    if gpu_monitor {
        if !Path::new("/usr/lib/libnvidia-ml.so").exists() {
            fs::symlink("/usr/lib/nvidia-418/libnvidia-ml.so", "/usr/lib/libnvidia-ml.so")?;
            return Ok(())
        }
    }
    return Ok(())
}