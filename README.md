# Rayflux

**Rayflux** is a ray tracer capable of rendering using either the **CPU** or **GPU**.
You can choose to display the result in real-time in a window or save it as a PNG image file.

## Features

- CPU ray tracing
- GPU ray tracing using wgpu
- Real-time rendering of the result in a window
- Result image export as PNG

## Usage

Run the ray tracer with your desired configuration:
```
cargo run -- --raytracer [cpu|gpu] --output [window|file]
```
