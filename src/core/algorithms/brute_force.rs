use crate::core::{MultiChannelInput, SDFData, SDFError, SDFAlgorithm};
use rayon::prelude::*;
use std::sync::Arc;

/// Simple brute-force SDF algorithm
/// O(n²) complexity but easy to understand and implement
pub struct BruteForce {
    threshold: u8,
    max_distance: f32,
}

impl BruteForce {
    pub fn new() -> Self {
        Self {
            threshold: 128,
            max_distance: 32.0,
        }
    }
    
    pub fn with_threshold(mut self, threshold: u8) -> Self {
        self.threshold = threshold;
        self
    }
    
    pub fn with_max_distance(mut self, max_distance: f32) -> Self {
        self.max_distance = max_distance;
        self
    }
}

impl SDFAlgorithm for BruteForce {
    fn process(&self, input: &MultiChannelInput) -> Result<SDFData, SDFError> {
        let (width, height) = input.dimensions()?;
        let mut sdf = SDFData::new(width, height, self.max_distance);
        
        // Get the primary channel (alpha or first available)
        let image = input.get_primary_channel()?;
        
        // Create a binary mask based on threshold
        let mask = Arc::new(create_binary_mask(&image, self.threshold));
        
        // Process each pixel in parallel
        let sdf_data: Vec<f32> = (0..height)
            .into_par_iter()
            .flat_map(|y| {
                let mask = Arc::clone(&mask);
                (0..width).into_par_iter().map(move |x| {
                    calculate_distance_brute_force(&mask, x, y, width, height, self.max_distance)
                })
            })
            .collect();
        
        sdf.data = sdf_data;
        Ok(sdf)
    }
    
    fn name(&self) -> &'static str {
        "brute-force"
    }
}

fn create_binary_mask(image: &image::DynamicImage, threshold: u8) -> Vec<bool> {
    let gray = image.to_luma8();
    gray.pixels()
        .map(|p| p[0] > threshold)
        .collect()
}

fn calculate_distance_brute_force(
    mask: &[bool],
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    max_distance: f32,
) -> f32 {
    let idx = (y * width + x) as usize;
    let is_inside = mask[idx];
    
    let mut min_distance = max_distance;
    
    // Search within a reasonable radius
    let search_radius = max_distance.ceil() as i32;
    
    for dy in -search_radius..=search_radius {
        for dx in -search_radius..=search_radius {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                let nidx = (ny as u32 * width + nx as u32) as usize;
                
                // Look for opposite pixels (inside looking for outside, or vice versa)
                if mask[nidx] != is_inside {
                    let distance = ((dx * dx + dy * dy) as f32).sqrt();
                    min_distance = min_distance.min(distance);
                }
            }
        }
    }
    
    // Signed distance: negative inside, positive outside
    if is_inside {
        -min_distance
    } else {
        min_distance
    }
}