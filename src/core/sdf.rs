use image::{GrayImage, Luma};
use nalgebra::Point2;

#[derive(Debug, Clone)]
pub struct SDFData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<f32>,
    pub max_distance: f32,
}

#[derive(Debug, Clone)]
pub struct SDF {
    pub data: SDFData,
    pub metadata: SDFMetadata,
}

#[derive(Debug, Clone)]
pub struct SDFMetadata {
    pub source_file: Option<String>,
    pub algorithm: String,
    pub threshold: u8,
    pub processing_time: Option<std::time::Duration>,
}

impl SDFData {
    pub fn new(width: u32, height: u32, max_distance: f32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            data: vec![max_distance; size],
            max_distance,
        }
    }
    
    pub fn get(&self, x: u32, y: u32) -> f32 {
        self.data[(y * self.width + x) as usize]
    }
    
    pub fn set(&mut self, x: u32, y: u32, value: f32) {
        self.data[(y * self.width + x) as usize] = value;
    }
    
    pub fn get_normalized(&self, x: u32, y: u32) -> f32 {
        let value = self.get(x, y);
        (value + self.max_distance) / (2.0 * self.max_distance)
    }
    
    pub fn to_grayscale_image(&self, normalize: bool) -> GrayImage {
        let mut img = GrayImage::new(self.width, self.height);
        
        for y in 0..self.height {
            for x in 0..self.width {
                let value = if normalize {
                    self.get_normalized(x, y)
                } else {
                    self.get(x, y) / self.max_distance
                };
                
                let pixel_value = (value.clamp(0.0, 1.0) * 255.0) as u8;
                img.put_pixel(x, y, Luma([pixel_value]));
            }
        }
        
        img
    }
    
    pub fn save_as_image<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), image::ImageError> {
        let img = self.to_grayscale_image(true);
        img.save(path)
    }
    
    pub fn analyze_regions(&self) -> Vec<Region> {
        // Simple region analysis - count distinct areas
        let mut regions = Vec::new();
        let threshold = 0.0;
        
        let mut visited = vec![false; (self.width * self.height) as usize];
        
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                if !visited[idx] && self.get(x, y) < threshold {
                    let region = self.flood_fill_region(x, y, &mut visited, threshold);
                    if region.pixel_count > 10 { // Ignore tiny regions
                        regions.push(region);
                    }
                }
            }
        }
        
        regions
    }
    
    fn flood_fill_region(&self, start_x: u32, start_y: u32, visited: &mut [bool], threshold: f32) -> Region {
        let mut stack = vec![(start_x, start_y)];
        let mut region = Region {
            center: Point2::new(0.0, 0.0),
            pixel_count: 0,
            bounds: (start_x, start_y, start_x, start_y),
        };
        
        while let Some((x, y)) = stack.pop() {
            let idx = (y * self.width + x) as usize;
            if visited[idx] {
                continue;
            }
            
            visited[idx] = true;
            region.pixel_count += 1;
            region.center.x += x as f32;
            region.center.y += y as f32;
            
            // Update bounds
            region.bounds.0 = region.bounds.0.min(x);
            region.bounds.1 = region.bounds.1.min(y);
            region.bounds.2 = region.bounds.2.max(x);
            region.bounds.3 = region.bounds.3.max(y);
            
            // Check neighbors
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    
                    if nx >= 0 && ny >= 0 && nx < self.width as i32 && ny < self.height as i32 {
                        let nx = nx as u32;
                        let ny = ny as u32;
                        let nidx = (ny * self.width + nx) as usize;
                        
                        if !visited[nidx] && self.get(nx, ny) < threshold {
                            stack.push((nx, ny));
                        }
                    }
                }
            }
        }
        
        if region.pixel_count > 0 {
            region.center.x /= region.pixel_count as f32;
            region.center.y /= region.pixel_count as f32;
        }
        
        region
    }
}

#[derive(Debug, Clone)]
pub struct Region {
    pub center: Point2<f32>,
    pub pixel_count: u32,
    pub bounds: (u32, u32, u32, u32), // min_x, min_y, max_x, max_y
}

impl SDF {
    pub fn from_raw_data(data: Vec<f32>, width: u32, height: u32, max_distance: f32) -> Self {
        Self {
            data: SDFData {
                width,
                height,
                data,
                max_distance,
            },
            metadata: SDFMetadata {
                source_file: None,
                algorithm: "unknown".to_string(),
                threshold: 128,
                processing_time: None,
            },
        }
    }
    
    pub fn save(&self, path: &std::path::Path, normalize: bool) -> Result<(), image::ImageError> {
        let img = self.data.to_grayscale_image(normalize);
        img.save(path)
    }
}