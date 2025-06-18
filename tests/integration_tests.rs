use sdf_maker::core::{
    MultiChannelInput, 
    algorithms::{BruteForce, JumpFloodingAlgorithm, SDFAlgorithm}
};
use image::{GrayImage, Luma};

fn create_test_circle(size: u32) -> MultiChannelInput {
    let mut img = GrayImage::new(size, size);
    
    let center = (size / 2) as f32;
    let radius = size as f32 / 4.0;
    
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let distance = (dx * dx + dy * dy).sqrt();
            
            let value = if distance <= radius { 255 } else { 0 };
            img.put_pixel(x, y, Luma([value]));
        }
    }
    
    MultiChannelInput::from_alpha(image::DynamicImage::ImageLuma8(img))
}

#[test]
fn test_brute_force_algorithm() {
    let input = create_test_circle(64);
    let algorithm = BruteForce::new().with_max_distance(32.0);
    
    let result = algorithm.process(&input);
    assert!(result.is_ok());
    
    let sdf = result.unwrap();
    assert_eq!(sdf.width, 64);
    assert_eq!(sdf.height, 64);
    assert_eq!(sdf.data.len(), 64 * 64);
    
    // Check that the center pixel (inside the circle) has negative distance
    let center_idx = (32 * 64 + 32) as usize;
    assert!(sdf.data[center_idx] < 0.0, "Center should be inside (negative distance)");
    
    // Check that a corner pixel (outside the circle) has positive distance
    let corner_idx = 0;
    assert!(sdf.data[corner_idx] > 0.0, "Corner should be outside (positive distance)");
}

#[test]
fn test_jfa_algorithm() {
    let input = create_test_circle(64);
    let algorithm = JumpFloodingAlgorithm::new().with_max_distance(32.0);
    
    let result = algorithm.process(&input);
    assert!(result.is_ok());
    
    let sdf = result.unwrap();
    assert_eq!(sdf.width, 64);
    assert_eq!(sdf.height, 64);
    assert_eq!(sdf.data.len(), 64 * 64);
}

#[test]
fn test_multi_channel_input() {
    let input = create_test_circle(32);
    
    // Test validation
    assert!(input.validate().is_ok());
    
    // Test dimensions
    let dims = input.dimensions();
    assert!(dims.is_ok());
    assert_eq!(dims.unwrap(), (32, 32));
    
    // Test channel queries
    assert!(input.has_alpha());
    assert!(!input.has_normal());
    assert!(!input.has_ao());
}

#[test]
fn test_sdf_data_operations() {
    use sdf_maker::core::SDFData;
    
    let mut sdf = SDFData::new(10, 10, 16.0);
    
    // Test set/get
    sdf.set(5, 5, -2.5);
    assert_eq!(sdf.get(5, 5), -2.5);
    
    // Test normalized access
    let normalized = sdf.get_normalized(5, 5);
    assert!(normalized >= 0.0 && normalized <= 1.0);
    
    // Test image conversion
    let img = sdf.to_grayscale_image(true);
    assert_eq!(img.width(), 10);
    assert_eq!(img.height(), 10);
}