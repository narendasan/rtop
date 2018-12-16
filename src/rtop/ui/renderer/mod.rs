mod cpu_tab;
#[cfg(feature = "gpu-monitor")]
mod gpu_tab;
mod render;

pub use self::render::render as render;