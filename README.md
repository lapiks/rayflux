# Rayflux

**Rayflux** is a ray tracer capable of rendering using either the **CPU** or **GPU**.
You can choose to display the result in real-time in a window or save it as a PNG image file.

## Features

- CPU ray tracing
- GPU ray tracing using wgpu
- Real-time rendering of the result in a window
- Result image export as PNG

## Project architecture

**Rayflux** is separated in three crates:
- **`rayflux`**
A library crate containing the core ray tracing engine (both CPU and GPU).

- **`rayflux_file`**
A **command-line executable** that renders a scene and outputs the result to an **image file**.  

- **`rayflux_window`**
A **real-time executable** that displays the rendered scene directly in a window.

## Usage

You can run the ray tracer using either the **rayflux_file** or **rayflux_window** crates. 
```
cargo run -p rayflux_file -- --raytracer [cpu|gpu]
cargo run -p rayflux_window -- --raytracer [cpu|gpu]
```
