use clap::Parser;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::{output::{file::FileApp, window::WindowApp, OutputMode}, raytracer::RaytracerType};

mod common;
mod raytracer;
mod output;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "gpu", value_enum)]
    raytracer: RaytracerType,
    #[arg(long, default_value = "window", value_enum)]
    output: OutputMode,
}

fn main() {
    let args = Args::parse();

    match args.output {
        OutputMode::Window => {
            let event_loop = EventLoop::new().unwrap();
            event_loop.set_control_flow(ControlFlow::Poll);
            let mut app = WindowApp::new(args.raytracer);
            let _ = event_loop.run_app(&mut app);
        }
        OutputMode::File => {
            let mut app = FileApp::default();
            app.run();
        }
    }
    
}
