mod render;
mod system_tab;
#[cfg(feature = "gpu-monitor")]
mod gpu_tab;

pub use self::render::render as render;
