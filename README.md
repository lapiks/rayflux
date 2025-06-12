# Rayflux

**Rayflux** is a raytracer capable of rendering using either the **CPU** or **GPU**.
You can choose to display the result in real-time in a window or save it as a PNG image file.

## Features

- CPU raytracing
- GPU raytracing using wgpu
- Real-time rendering in a window
- Image export as PNG

## Usage

Run the raytracer with your desired configuration:
```
cargo run -- --raytracer [cpu|gpu] --output [window|file]
```
