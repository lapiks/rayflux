mod app;

use clap::Parser;
use rayflux::raytracer::RaytracerType;

use crate::app::App;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "cpu", value_enum)]
    raytracer: RaytracerType,
}

fn main() {
    let args = Args::parse();
    let mut app = App::new(args.raytracer);
    app.run();
}
