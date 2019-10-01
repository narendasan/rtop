extern crate dirs;
use std::env;
use std::os::unix::fs;
use std::process::Command;
use std::str;

//Apologies if you're trying to figure this out, ping me (@narendasan) if you need help :)
fn main() -> std::io::Result<()> {
    let gpu_monitor = env::var_os("CARGO_FEATURE_GPU_MONITOR").is_some();
    if gpu_monitor {
        let ldconfig = Command::new("ldconfig").arg("-p")
                                               .output()
                                               .expect("Failed to run ldconfig");
        let ldc_output = String::from_utf8_lossy(&(ldconfig.stdout[..])); 
        let nvml_installed = ldc_output.split("\n").collect::<Vec<&str>>()
                                      .iter()
                                      .map(|x| x.split(" ").collect::<Vec<&str>>()[0])
                                      .map(|x| x.replace("\t", ""))
                                      .fold(false, |acc, lib| lib == "libnvidia-ml.so" || acc);
        if !nvml_installed {
            let nvidia_driver_version = match Command::new("cat").arg("/proc/driver/nvidia/version").output() {
                Ok(out) => {
                    let nvidia_driver_version_info = match str::from_utf8(&out.stdout) {
                        Ok(out) => out.to_string(),
                        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not parse driver version"))
                    };
                    let first_line = nvidia_driver_version_info.split(".").map(|x| x.to_string()).collect::<Vec<String>>()[0].clone();
                    match first_line.split("  ").map(|x| x.to_string()).collect::<Vec<String>>().last().cloned() {
                        Some(ver) => ver,
                        None => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not parse driver version"))
                    }
                }
                Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not find NVIDIA Driver verion"))
            };
            println!("cargo:rustc-env=LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/lib/nvidia-{}/libnvidia-ml.so", nvidia_driver_version);

            let home_dir = match dirs::home_dir() {
                Some(p) => format!("{}", p.display()),
                None => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to find path to home directory"))
            };
            fs::symlink(&format!("/usr/lib/nvidia-{}/libnvidia-ml.so", nvidia_driver_version), &format!("{}/.local/lib/libnvidia-ml.so", home_dir))?;
        }
    }
    return Ok(())
}
