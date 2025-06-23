use clap::Parser;
use rayflux::raytracer::RaytracerType;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::App;

mod app;
mod ui;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "cpu", value_enum)]
    raytracer: RaytracerType,
}

fn main() {
    let args = Args::parse();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(args.raytracer);
    let _ = event_loop.run_app(&mut app);
}
