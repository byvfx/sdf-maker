# Claude.md - 2D SDF Maker in Rust

## Project Overview
A Rust program that converts grayscale images into 2D Signed Distance Fields (SDFs) for use in game engines. The tool takes texture inputs and generates distance field data for efficient rendering, collision detection, and visual effects.

## Core Functionality

### Multi-Channel Input Processing
- **Traditional Alpha Mode**: Standard grayscale/alpha to SDF conversion
- **Enhanced Multi-Channel Mode**: Combine multiple texture maps for superior results
- **Artistic Control Mode**: Manual weighting and blending of input channels

### Supported Input Channels

#### Primary Channels
- **Alpha/Mask**: Traditional foreground/background separation
- **Grayscale**: Luminance-based shape detection
- **Red/Green/Blue**: Individual color channel processing

#### Enhancement Channels
- **Normal Maps**: Surface detail and curvature information
- **Ambient Occlusion (AO)**: Natural distance-like information
- **Curvature Maps**: Surface geometry understanding
- **Height/Displacement**: Surface elevation data
- **Custom Channels**: Artist-painted influence maps, feature maps, blend maps

### Channel Combination Strategies
- **Blending Modes**: Multiply, Add, Screen, Overlay, Custom operations
- **Processing Pipelines**: Enhanced edge detection, AO-biased generation, multi-layer approach, artist-controlled

### Advanced Processing Features
- **Intelligent Channel Analysis**: Auto-detection, content analysis, quality assessment
- **Feature Preservation**: Edge sharpness control, detail enhancement, noise reduction
- **Multi-Scale Processing**: Hierarchical SDFs, detail injection, progressive enhancement

### SDF Generation Methods

#### Traditional Algorithms
- Brute Force Method
- Jump Flooding Algorithm (JFA)
- Euclidean Distance Transform

#### Enhanced Multi-Channel Algorithms
- Weighted Distance Transform
- Feature-Aware JFA
- Gradient-Enhanced Transform
- AO-Guided Generation
- Curvature-Preserving Transform

### Output Options
- Raw SDF Data, Normalized SDF, Signed Distance
- Multi-Layer SDFs (base shape + details)
- Export Formats: PNG/JPEG, EXR, Custom texture formats, Multi-file exports

## Technical Architecture

### Core Components
1. **Multi-Channel Image Processor**: Load, validate, and preprocess channels
2. **Channel Analysis Module**: Content analysis and optimization recommendations
3. **Enhanced SDF Generator**: Multi-channel algorithms and blending
4. **GUI Module**: UI state management and real-time preview
5. **Batch Processing Module**: Parallel job scheduling and progress tracking
6. **Configuration Management**: TOML/JSON parsing and preset management

### Key Implementation Details

#### Multi-Channel Input Handler
```rust
struct MultiChannelInput {
    alpha: Option<DynamicImage>,
    normal: Option<DynamicImage>,
    ao: Option<DynamicImage>,
    curvature: Option<DynamicImage>,
    height: Option<DynamicImage>,
    custom_channels: HashMap<String, DynamicImage>,
    weights: ChannelWeights,
    blend_mode: BlendMode,
}
```

#### Parallel Processing
- Thread pool management with rayon
- SIMD optimization for distance calculations
- Streaming processing for large images
- Memory-efficient tile-based processing

#### Error Handling
- Comprehensive error types with recovery suggestions
- Input validation pipeline
- Dimension and format checking

## Dependencies

### Essential Crates
```toml
[dependencies]
image = "0.24"           # Image loading/saving
rayon = "1.7"           # Parallel processing
anyhow = "1.0"          # Error handling
clap = "4.0"            # Command-line interface
serde = "1.0"           # Configuration serialization
toml = "0.8"            # TOML config files
egui = "0.24"           # GUI framework
```

## User Interfaces

### GUI Application

#### Framework
- **egui**: Immediate mode GUI, excellent for Rust integration

#### Features
- **Drag & Drop**: Direct image file dropping onto interface
- **Multi-Channel Input**: Load multiple texture maps simultaneously
- **Channel Visualization**: Preview each input channel separately
- **Real-time Preview**: Show original and SDF side-by-side with live updates
- **Interactive Parameters**: Sliders and controls for all settings
- **Channel Mixing**: Visual blend mode controls and weight sliders
- **Feature Highlighting**: Overlay detected features on preview
- **Batch Processing**: Queue multiple file sets with progress tracking
- **Presets**: Save/load common parameter configurations
- **Export Options**: Visual format selection with previews

#### Layout Design
```
┌─────────────────────────────────────────────────────────────────────────────────┐
│ File Menu | Edit | View | Tools | Help                                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│ [Open Set] [Save] [Batch] [Presets ▼]                           [Process SDF]   │
├─────────────────┬───────────────────────────────────────────────────────────────┤
│                 │                                                               │
│   Input Channels│                    Preview Area                               │
│                 │  ┌─────────┬─────────┬─────────┬─────────┬─────────┐          │
│ □ Alpha    [📁] │  │ Alpha   │ Normal  │   AO    │Curvature│   SDF   │          │
│ □ Normal   [📁] │  │         │         │         │         │         │          │
│ □ AO       [📁] │  │ [Image] │ [Image] │ [Image] │ [Image] │ [Image] │          │
│ □ Curvature[📁] │  │         │         │         │         │         │          │
│                 │  └─────────┴─────────┴─────────┴─────────┴─────────┘          │
│   Parameters    │                                                               │
│                 │  Channel Weights:                                             │
│ Method: Enhanced│  Alpha:     ████████░░ 80%                                    │
│ ├─────────────┤ │  Normal:    ██████░░░░ 60%                                    │
│                 │  AO:        ████░░░░░░ 40%                                    │
│ Threshold: 128  │  Curvature: ██░░░░░░░░ 20%                                    │
│ ├─────────────┤ │                                                               │
│                 │  Blend Mode: Multiply ▼     Feature Detection: Auto ▼        │
│ Max Dist:  32   │                                                               │
│ ├─────────────┤ │  Progress: ████████░░ 80%                                     │
│                 │  Status: Processing enhanced SDF...                          │
│ □ Multi-layer   │                                                               │
│ □ Preserve detail│                                                              │
└─────────────────┴───────────────────────────────────────────────────────────────┘
```

### Command Line Interface

#### Basic Usage
```bash
# Single file
sdf-maker --alpha diffuse.png --output sdf.png

# Multi-channel
sdf-maker --alpha diffuse.png --normal normal_map.png --ao ao.png --output enhanced_sdf.png

# Batch processing
sdf-maker --batch textures/ --output sdf_output/
```

#### Key Parameters
- Channel inputs: `--alpha`, `--normal`, `--ao`, `--curvature`, `--height`
- Channel weights: `--weight-alpha`, `--weight-normal`, etc.
- Processing: `--method`, `--quality`, `--preserve-features`
- Batch: `--batch`, `--batch-config`, `--preserve-structure`
- Performance: `--jobs`, `--progress`, `--verbose`

### Configuration Files
```toml
[input]
directory = "textures/"
auto_detect_channels = true

[channels]
enabled = ["alpha", "normal", "ao"]
alpha = { pattern = "*_diffuse.*", weight = 1.0 }
normal = { pattern = "*_normal.*", weight = 0.7 }

[processing]
method = "enhanced-jfa"
quality = "high"

[output]
directory = "sdf_output/"
format = "png"
```

## Implementation Phases

### Phase 1: Core Foundation
- Set up Rust project structure
- Implement basic image loading/saving
- Create simple SDF algorithm
- Basic CLI argument parsing

### Phase 2: CLI Batch Processing
- Directory traversal and file filtering
- Parallel batch processing
- Configuration file support
- Error handling and recovery

### Phase 3: GUI Development
- GUI framework setup (egui)
- Real-time parameter adjustment
- Image preview functionality
- File open/save dialogs

### Phase 4: Advanced Features
- Multi-channel processing
- Feature preservation algorithms
- Preset system
- Cross-platform testing

## Technical Considerations

### Performance
- **Algorithm Complexity**: Brute Force O(n²), JFA O(n log n)
- **Memory Management**: Streaming for large images, memory pooling
- **CPU Optimization**: SIMD vectorization, cache-friendly algorithms
- **Parallel Processing**: Work-stealing thread pools, dynamic load balancing

### Cross-Platform
- Platform-specific optimizations (AVX2/SSE for x86, NEON for ARM)
- Resource detection and adaptive processing
- Windows, macOS, Linux compatibility

## Quality Assurance

### Testing Strategy
- **Unit Tests**: Test individual algorithms
- **Integration Tests**: End-to-end pipeline testing
- **Performance Tests**: Benchmark against reference implementations
- **Visual Tests**: Compare output quality with known-good results

### Validation Methods
- **Reference Images**: Use well-known test cases
- **Cross-validation**: Compare different algorithm outputs
- **Game Engine Testing**: Verify compatibility with target engines

## Success Metrics

### Functionality Goals
- Accurate SDF generation from single and multi-channel inputs
- Support for common image formats
- Reasonable performance on typical game textures
- Simple, intuitive interface

### Performance Targets
- Small Images (256x256): < 200ms
- Medium Images (1024x1024): < 3 seconds
- Large Images (2048x2048): < 15 seconds
- Memory Usage: < 2x input image size

## Future Enhancements
- 3D SDF Support
- GPU Acceleration
- Real-time Preview
- Advanced Filtering
- Specialized compression

---

*This document serves as a living specification for the 2D SDF maker project.*