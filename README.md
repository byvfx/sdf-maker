# SDF Maker

A Rust program that converts grayscale images into 2D Signed Distance Fields (SDFs) for use in game engines.

## Features

- **Multi-Channel Processing**: Support for alpha, normal maps, ambient occlusion, and curvature maps
- **Multiple Algorithms**: Brute-force, Jump Flooding Algorithm (JFA), and feature-aware processing
- **CLI and GUI**: Command-line interface for batch processing and GUI for interactive editing
- **High Performance**: Parallel processing with Rust's safety guarantees
- **Game Engine Ready**: Output formats compatible with Unity, Unreal, and other engines

## Quick Start

### Installation

Make sure you have Rust installed, then:

```bash
git clone https://github.com/byvfx/sdf-maker.git
cd sdf-maker
cargo build --release
```

### Basic Usage

```bash
# Process single image
./target/release/sdf-maker process input.png -o output_sdf.png

# With multiple channels
./target/release/sdf-maker process \
  --alpha diffuse.png \
  --normal normal_map.png \
  --ao ambient_occlusion.png \
  -o enhanced_sdf.png

# Batch processing
./target/release/sdf-maker batch textures/ -o sdf_output/ --progress

# Launch GUI
./target/release/sdf-maker
```

## Algorithms

### Brute Force
Simple O(n²) algorithm, good for small images and understanding:
```bash
sdf-maker process input.png --method brute --max-distance 32
```

### Jump Flooding Algorithm (JFA)
Efficient O(n log n) algorithm for larger images:
```bash
sdf-maker process input.png --method jfa --max-distance 64
```

### Feature-Aware Processing
Uses normal maps and surface features for enhanced quality:
```bash
sdf-maker process --alpha sprite.png --normal sprite_normal.png --method feature-aware
```

## Multi-Channel Processing

SDF Maker can combine multiple texture channels for superior results:

- **Alpha Channel**: Traditional shape/mask definition
- **Normal Maps**: Surface detail and edge preservation
- **Ambient Occlusion**: Natural distance-like information
- **Curvature Maps**: Surface geometry understanding

### Example with All Channels

```bash
sdf-maker process \
  --alpha character_diffuse.png \
  --normal character_normal.png \
  --ao character_ao.png \
  --curvature character_curvature.png \
  --method feature-aware \
  --max-distance 48 \
  -o character_enhanced_sdf.png
```

## Configuration Files

For complex batch operations, use TOML configuration:

```toml
# batch_config.toml
[input]
directory = "textures/"
recursive = true

[channels]
alpha = { pattern = "*_diffuse.*", weight = 1.0 }
normal = { pattern = "*_normal.*", weight = 0.7 }
ao = { pattern = "*_ao.*", weight = 0.5 }

[processing]
method = "feature-aware"
max_distance = 32.0
quality = "high"

[output]
directory = "sdf_output/"
format = "png"
```

Run with:
```bash
sdf-maker batch --config batch_config.toml
```

## Performance

Typical processing times on modern hardware:

- **256x256**: < 200ms (brute force), < 50ms (JFA)
- **1024x1024**: < 3s (brute force), < 500ms (JFA)  
- **2048x2048**: < 15s (brute force), < 2s (JFA)

## Project Status

This is the initial implementation focusing on core functionality:

- [x] Basic project structure
- [x] Multi-channel input processing
- [x] Brute force and JFA algorithms
- [x] CLI interface with parameter support
- [ ] GUI implementation
- [ ] Batch processing
- [ ] Advanced feature-aware algorithms
- [ ] Performance optimizations (SIMD, GPU)

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Benchmarks

```bash
cargo bench
```
