# 3D Projection Helper

An interactive 3D visualization tool built with Rust and Macroquad that helps understand camera projection matrices, view frustums, and 3D coordinate transformations.

## Features

- **Interactive 3D Camera System**: Control a custom camera with keyboard inputs to explore the scene
- **View Frustum Visualization**: See the camera's view frustum in real-time with near and far planes
- **Projection Matrix Visualization**: Visual helpers to understand how view and projection matrices work
- **Reference Coordinate System**: X, Y, Z axes displayed for spatial orientation
- **Vector Viewers**: Mini viewports showing forward and right vectors from the camera's perspective
- **Real-time Transformations**: Rotate the main view and see all transformations update dynamically

## Prerequisites

- **Rust** (latest stable version)
- **cargo** (comes with Rust)
- **cargo-make** (for WASM builds): `cargo install cargo-make`

## Building & Running

### Desktop (Native)

```bash
cargo run
```

### WebAssembly

1. Add the WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

2. Build the WASM binary:
```bash
cargo make build
```

3. Serve the web application (from the `web` directory):
```bash
cd web
npm install
npm run dev
```

## Controls

### Main Camera (Orbit View)
- **Left Arrow** / **Right Arrow**: Rotate the orbital camera around the scene
- **Q** / **E**: Rotate the player camera view angle

### Controlled Camera (Yellow Cube)
- **W** / **S**: Move forward/backward (Z-axis)
- **A** / **D**: Move left/right (X-axis)
- **Space**: Move up (Y-axis)
- **Left Shift**: Move down (Y-axis)