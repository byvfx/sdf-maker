use sdf_maker::core::{MultiChannelInput, SDFAlgorithm};
use sdf_maker::core::algorithms::{BruteForce, JumpFloodingAlgorithm};

fn main() -> anyhow::Result<()> {
    println!("SDF Maker - Verification Test");
    println!("=============================");
    
    // Test with different shapes
    test_shape("Square", create_square_image(128, 128, 40)?)?;
    test_shape("Circle", create_circle_image(128, 128, 40)?)?;
    test_shape("Ring", create_ring_image(128, 128, 40, 20)?)?;
    
    println!("\nAll tests completed!");
    Ok(())
}

fn test_shape(name: &str, img: image::DynamicImage) -> anyhow::Result<()> {
    println!("\nTesting shape: {}", name);
    
    // Save the input image
    let input_filename = format!("test_{}_input.png", name.to_lowercase());
    img.save(&input_filename)?;
    
    // Create multi-channel input
    let input = MultiChannelInput::new().with_alpha(img);
    
    // Test both algorithms
    let algorithms: Vec<(&str, Box<dyn SDFAlgorithm>)> = vec![
        ("brute_force", Box::new(BruteForce::new().with_max_distance(64.0))),
        ("jfa", Box::new(JumpFloodingAlgorithm::new().with_max_distance(64.0))),
    ];
    
    for (algo_name, algorithm) in algorithms {
        let start = std::time::Instant::now();
        let sdf = algorithm.process(&input)?;
        let elapsed = start.elapsed();
        
        println!("  {} completed in: {:?}", algo_name, elapsed);
        
        // Save the SDF
        let output_filename = format!("test_{}_{}.png", name.to_lowercase(), algo_name);
        sdf.save_as_image(&output_filename)?;
        
        // Verify some basic properties
        let (width, height) = (sdf.width, sdf.height);
        let center_x = width / 2;
        let center_y = height / 2;
        
        // Sample some points
        println!("    Center value: {:.2}", sdf.get(center_x, center_y));
        println!("    Corner value: {:.2}", sdf.get(0, 0));
        println!("    Edge value: {:.2}", sdf.get(center_x, 0));
    }
    
    Ok(())
}

fn create_square_image(width: u32, height: u32, size: u32) -> anyhow::Result<image::DynamicImage> {
    let mut img = image::RgbaImage::new(width, height);
    
    let left = (width - size) / 2;
    let top = (height - size) / 2;
    let right = left + size;
    let bottom = top + size;
    
    for y in 0..height {
        for x in 0..width {
            if x >= left && x < right && y >= top && y < bottom {
                img.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            } else {
                img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
            }
        }
    }
    
    Ok(image::DynamicImage::ImageRgba8(img))
}

fn create_circle_image(width: u32, height: u32, radius: u32) -> anyhow::Result<image::DynamicImage> {
    let mut img = image::RgbaImage::new(width, height);
    
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let radius_f = radius as f32;
    
    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance <= radius_f {
                img.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            } else {
                img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
            }
        }
    }
    
    Ok(image::DynamicImage::ImageRgba8(img))
}

fn create_ring_image(width: u32, height: u32, outer_radius: u32, inner_radius: u32) -> anyhow::Result<image::DynamicImage> {
    let mut img = image::RgbaImage::new(width, height);
    
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let outer_f = outer_radius as f32;
    let inner_f = inner_radius as f32;
    
    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance <= outer_f && distance >= inner_f {
                img.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            } else {
                img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
            }
        }
    }
    
    Ok(image::DynamicImage::ImageRgba8(img))
}