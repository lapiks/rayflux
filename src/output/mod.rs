use clap::ValueEnum;

pub mod window;
pub mod file;

/// The output mode for the ray tacer
#[derive(ValueEnum, Clone, Debug)]
pub enum OutputMode {
    /// Rendering directly on a window
    Window,
    /// Rendering to an image file
    File,
}