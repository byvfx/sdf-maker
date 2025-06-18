use sdf_maker::core::{MultiChannelInput, SDFAlgorithm};
use sdf_maker::core::algorithms::{BruteForce, JumpFloodingAlgorithm};

fn main() -> anyhow::Result<()> {
    println!("SDF Maker - Simple Test");
    println!("======================");
    
    // Create a simple test image (white circle on black background)
    let width = 64;
    let height = 64;
    let mut img = image::RgbaImage::new(width, height);
    
    // Draw a white circle in the center
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let radius = 20.0;
    
    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance <= radius {
                img.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            } else {
                img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
            }
        }
    }
    
    // Save the test image
    img.save("test_circle.png")?;
    println!("Created test image: test_circle.png");
    
    // Convert to DynamicImage
    let dynamic_img = image::DynamicImage::ImageRgba8(img);
    
    // Create multi-channel input
    let input = MultiChannelInput::new()
        .with_alpha(dynamic_img);
    
    // Test with brute force algorithm
    println!("\nTesting Brute Force algorithm...");
    let brute_force = BruteForce::new()
        .with_threshold(128)
        .with_max_distance(32.0);
    
    let start = std::time::Instant::now();
    let sdf_result = brute_force.process(&input)?;
    let elapsed = start.elapsed();
    println!("Brute Force completed in: {:?}", elapsed);
    
    // Save the result
    sdf_result.save_as_image("sdf_brute_force.png")?;
    println!("Saved SDF result: sdf_brute_force.png");
    
    // Test with JFA algorithm
    println!("\nTesting Jump Flooding algorithm...");
    let jfa = JumpFloodingAlgorithm::new()
        .with_threshold(128)
        .with_max_distance(32.0);
    
    let start = std::time::Instant::now();
    let sdf_result = jfa.process(&input)?;
    let elapsed = start.elapsed();
    println!("JFA completed in: {:?}", elapsed);
    
    // Save the result
    sdf_result.save_as_image("sdf_jfa.png")?;
    println!("Saved SDF result: sdf_jfa.png");
    
    println!("\nTest completed successfully!");
    
    Ok(())
}