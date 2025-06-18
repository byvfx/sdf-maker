use crate::core::{MultiChannelInput, algorithms, SDF, SDFError};
use anyhow::Result;
use std::path::PathBuf;
use std::time::Instant;

/// Process a single file with multi-channel support
pub async fn process_single(
    input: PathBuf,
    output: Option<PathBuf>,
    alpha: Option<PathBuf>,
    normal: Option<PathBuf>,
    ao: Option<PathBuf>,
    curvature: Option<PathBuf>,
    method: &str,
    threshold: u8,
    max_distance: f32,
) -> Result<()> {
    let start_time = Instant::now();
    
    println!("Processing SDF with method: {}", method);
    
    // Build multi-channel input
    let mut channels = MultiChannelInput::new();
    
    // Load channels
    if let Some(alpha_path) = alpha {
        println!("Loading alpha channel: {:?}", alpha_path);
        channels.load_alpha(alpha_path)?;
    } else {
        // Use the main input as alpha channel
        println!("Loading input as alpha channel: {:?}", input);
        channels.load_alpha(&input)?;
    }
    
    if let Some(normal_path) = normal {
        println!("Loading normal channel: {:?}", normal_path);
        channels.load_normal(normal_path)?;
    }
    
    if let Some(ao_path) = ao {
        println!("Loading AO channel: {:?}", ao_path);
        channels.load_ao(ao_path)?;
    }
    
    if let Some(curvature_path) = curvature {
        println!("Loading curvature channel: {:?}", curvature_path);
        channels.load_curvature(curvature_path)?;
    }
    
    // Validate input
    channels.validate()?;
    let (width, height) = channels.dimensions()?;
    println!("Image dimensions: {}x{}", width, height);
    
    // Create and configure algorithm
    let algorithm = match method.to_lowercase().as_str() {
        "brute" | "brute-force" => {
            Box::new(algorithms::BruteForce::new()
                .with_threshold(threshold)
                .with_max_distance(max_distance)) as Box<dyn algorithms::SDFAlgorithm>
        },
        "jfa" | "jump-flooding" => {
            Box::new(algorithms::JumpFloodingAlgorithm::new()
                .with_threshold(threshold)
                .with_max_distance(max_distance)) as Box<dyn algorithms::SDFAlgorithm>
        },
        "feature-aware" | "feature-aware-jfa" => {
            Box::new(algorithms::FeatureAwareJFA::new()
                .with_threshold(threshold)
                .with_max_distance(max_distance)) as Box<dyn algorithms::SDFAlgorithm>
        },
        _ => {
            return Err(SDFError::ProcessingFailed {
                reason: format!("Unknown algorithm: {}", method),
            }.into());
        }
    };
    
    // Process SDF
    println!("Generating SDF...");
    let sdf_data = algorithm.process(&channels)?;
    
    // Create SDF with metadata
    let mut sdf = SDF::from_raw_data(
        sdf_data.data,
        sdf_data.width,
        sdf_data.height,
        sdf_data.max_distance,
    );
    
    sdf.metadata.algorithm = algorithm.name().to_string();
    sdf.metadata.threshold = threshold;
    sdf.metadata.processing_time = Some(start_time.elapsed());
    sdf.metadata.source_file = Some(input.to_string_lossy().to_string());
    
    // Determine output path
    let output_path = if let Some(out) = output {
        out
    } else {
        let mut out = input.clone();
        out.set_extension("sdf.png");
        out
    };
    
    // Save result
    println!("Saving SDF to: {:?}", output_path);
    sdf.save(&output_path, true)?;
    
    let elapsed = start_time.elapsed();
    println!("Processing completed in {:.2}s", elapsed.as_secs_f64());
    
    Ok(())
}

