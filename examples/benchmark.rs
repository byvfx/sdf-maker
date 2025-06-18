use sdf_maker::core::{MultiChannelInput, SDFAlgorithm};
use sdf_maker::core::algorithms::{BruteForce, JumpFloodingAlgorithm};
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    println!("SDF Maker - Performance Benchmark");
    println!("=================================\n");
    
    let sizes = vec![64, 128, 256, 512];
    
    println!("{:<10} {:<15} {:<15} {:<10}", "Size", "Brute Force", "JFA", "Speedup");
    println!("{}", "-".repeat(50));
    
    for size in sizes {
        let img = create_test_image(size)?;
        let input = MultiChannelInput::new().with_alpha(img);
        
        // Benchmark brute force
        let brute_force = BruteForce::new().with_max_distance(size as f32 / 4.0);
        let bf_time = benchmark_algorithm(&brute_force, &input, 3)?;
        
        // Benchmark JFA
        let jfa = JumpFloodingAlgorithm::new().with_max_distance(size as f32 / 4.0);
        let jfa_time = benchmark_algorithm(&jfa, &input, 3)?;
        
        let speedup = bf_time.as_secs_f64() / jfa_time.as_secs_f64();
        
        println!("{:<10} {:<15} {:<15} {:<10.2}x", 
            format!("{}x{}", size, size),
            format!("{:.3}ms", bf_time.as_secs_f64() * 1000.0),
            format!("{:.3}ms", jfa_time.as_secs_f64() * 1000.0),
            speedup
        );
    }
    
    println!("\nBenchmark completed!");
    Ok(())
}

fn benchmark_algorithm(algorithm: &dyn SDFAlgorithm, input: &MultiChannelInput, iterations: u32) -> anyhow::Result<Duration> {
    // Warm up
    algorithm.process(input)?;
    
    // Benchmark
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        algorithm.process(input)?;
    }
    let total_time = start.elapsed();
    
    Ok(total_time / iterations)
}

fn create_test_image(size: u32) -> anyhow::Result<image::DynamicImage> {
    let mut img = image::RgbaImage::new(size, size);
    
    // Create a complex pattern with multiple shapes
    let center_x = size as f32 / 2.0;
    let center_y = size as f32 / 2.0;
    
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            // Circle in the center
            let in_circle = distance <= size as f32 / 4.0;
            
            // Square in corners
            let in_square = (x < size / 4 || x >= 3 * size / 4) && 
                           (y < size / 4 || y >= 3 * size / 4);
            
            if in_circle || in_square {
                img.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            } else {
                img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
            }
        }
    }
    
    Ok(image::DynamicImage::ImageRgba8(img))
}