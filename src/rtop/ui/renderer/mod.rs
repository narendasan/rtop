#[cfg(feature = "gpu-monitor")]
mod gpu_tab;
mod render;
mod system_tab;

pub use self::render::render;
