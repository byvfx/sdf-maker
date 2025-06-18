use crate::core::{MultiChannelInput, SDFData, SDFError, SDFAlgorithm};
use rayon::prelude::*;

/// Jump Flooding Algorithm for efficient SDF generation
/// O(n log n) complexity with better performance for large images
pub struct JumpFloodingAlgorithm {
    threshold: u8,
    max_distance: f32,
    subpixel_precision: bool,
}

impl JumpFloodingAlgorithm {
    pub fn new() -> Self {
        Self {
            threshold: 128,
            max_distance: 32.0,
            subpixel_precision: false,
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
    
    pub fn with_subpixel_precision(mut self, enable: bool) -> Self {
        self.subpixel_precision = enable;
        self
    }
}

impl SDFAlgorithm for JumpFloodingAlgorithm {
    fn process(&self, input: &MultiChannelInput) -> Result<SDFData, SDFError> {
        let (width, height) = input.dimensions()?;
        let mut sdf = SDFData::new(width, height, self.max_distance);
        
        // Get the primary channel
        let image = input.get_primary_channel()?;
        
        // Initialize seed points
        let seeds = initialize_seeds(&image, self.threshold, width, height);
        
        // Run JFA
        let result = jump_flooding(&seeds, width, height, self.max_distance, self.threshold);
        
        sdf.data = result;
        Ok(sdf)
    }
    
    fn name(&self) -> &'static str {
        "jump-flooding"
    }
}

#[derive(Debug, Clone, Copy)]
struct Seed {
    x: i32,
    y: i32,
}

const NO_SEED: Seed = Seed { x: -1, y: -1 };

fn initialize_seeds(
    image: &image::DynamicImage,
    threshold: u8,
    width: u32,
    height: u32,
) -> Vec<Seed> {
    let gray = image.to_luma8();
    let mut seeds = vec![NO_SEED; (width * height) as usize];
    
    // Find edge pixels as seeds
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let pixel = gray.get_pixel(x, y)[0];
            let is_inside = pixel > threshold;
            
            // Check if this is an edge pixel
            let mut is_edge = false;
            
            // Check 4-connected neighbors
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                    let neighbor_pixel = gray.get_pixel(nx as u32, ny as u32)[0];
                    let neighbor_inside = neighbor_pixel > threshold;
                    
                    if is_inside != neighbor_inside {
                        is_edge = true;
                        break;
                    }
                }
            }
            
            if is_edge {
                seeds[idx] = Seed { x: x as i32, y: y as i32 };
            }
        }
    }
    
    seeds
}

fn jump_flooding(
    seeds: &[Seed],
    width: u32,
    height: u32,
    max_distance: f32,
    _threshold: u8,
) -> Vec<f32> {
    let mut current_seeds = seeds.to_vec();
    let mut next_seeds = vec![NO_SEED; (width * height) as usize];
    
    // Calculate number of passes needed
    let max_dim = width.max(height);
    let mut step = 1;
    while step < max_dim {
        step *= 2;
    }
    
    // JFA passes
    while step >= 1 {
        // Clear next generation
        next_seeds.fill(NO_SEED);
        
        // Process each pixel
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
                let mut best_seed = current_seeds[idx];
                let mut best_distance = f32::INFINITY;
                
                // Check all neighbors at current step distance
                for dy in (-1..=1).map(|d| d * step as i32) {
                    for dx in (-1..=1).map(|d| d * step as i32) {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        
                        if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                            let nidx = (ny as u32 * width + nx as u32) as usize;
                            let neighbor_seed = current_seeds[nidx];
                            
                            if neighbor_seed.x >= 0 && neighbor_seed.y >= 0 {
                                let distance = calculate_distance(
                                    x as i32,
                                    y as i32,
                                    neighbor_seed.x,
                                    neighbor_seed.y,
                                );
                                
                                if distance < best_distance {
                                    best_distance = distance;
                                    best_seed = neighbor_seed;
                                }
                            }
                        }
                    }
                }
                
                next_seeds[idx] = best_seed;
            }
        }
        
        // Swap buffers
        std::mem::swap(&mut current_seeds, &mut next_seeds);
        step /= 2;
    }
    
    // Convert seeds to distance values
    current_seeds
        .par_iter()
        .enumerate()
        .map(|(idx, seed)| {
            let x = (idx as u32) % width;
            let y = (idx as u32) / width;
            
            if seed.x >= 0 && seed.y >= 0 {
                let distance = calculate_distance(x as i32, y as i32, seed.x, seed.y);
                
                // For now, just return unsigned distance
                // Sign determination should be done with the original image data
                distance.min(max_distance)
            } else {
                max_distance
            }
        })
        .collect()
}

fn calculate_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    let dx = (x2 - x1) as f32;
    let dy = (y2 - y1) as f32;
    (dx * dx + dy * dy).sqrt()
}