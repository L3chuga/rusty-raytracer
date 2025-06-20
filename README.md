# Rusty Raytracer

Rusty Raytracer is a simple ray tracing project implemented in Rust, inspired by Peter Shirley's *"Ray Tracing in One Weekend"*. This was my first project ever using Rust, I used it as a way to learn the language and explore foundational concepts in computer graphics.

## Features

- **Materials**: Supports Lambertian (diffuse), Metal (reflective), and Dielectric (refractive) materials.
- **Customizable Camera**: Allows for different camera angles, positions, and field of view (FOV).
- **Random Sampling**: Implements random sampling for realistic rendering.
- **Scene Composition**: Renders scenes with multiple spheres and materials.

## Example Output

The raytracer is capable of rendering scenes like this:

![Rusty Ray Tracer Output as png](https://github.com/L3chuga/rusty-raytracer/blob/master/example_rrt_render.png)

## Getting Started

### Prerequisites

- Rust (edition 2021 or later)
- A basic understanding of Rust and ray tracing concepts is helpful but not required.

### Installation

1. Clone the repository:
```bash
git clone https://github.com/L3chuga/rusty-raytracer.git
cd rusty-raytracer
```
2. Build the project:
```bash
cargo build --release
```
3. Run the ray tracer:
```bash
cargo run --release
```
The rendered image will be saved as `image.ppm` in the project directory. With the default configuration (low resolution and low detail) it takes around 11 seconds to render a full image in my low spec laptop. You can freely change the constants in main.rs to get better images at a larger rendering time cost. You can use online .ppm viewers to see the finished images but I recommend using a vscode .ppm extension as this way you can see the image being generated in real time.

## Acknowledgments

- Inspired by Peter Shirley's _"Ray Tracing in One Weekend"_.

## License

This project is licensed under the MIT License.
