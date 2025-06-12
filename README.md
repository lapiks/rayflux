# Rayflux

**Rayflux** is a raytracer capable of rendering using either the **CPU** or **GPU**.
You can choose to display the result in real-time in a window or save it as a PNG image file.

## Features

- CPU raytracing
- GPU raytracing using wgpu
- Real-time rendering of the result in a window
- Result image export as PNG

## Usage

Run the raytracer with your desired configuration:
```
cargo run -- --raytracer [cpu|gpu] --output [window|file]
```
