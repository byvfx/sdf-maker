use crate::core::SDFError;
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct MultiChannelInput {
    pub alpha: Option<DynamicImage>,
    pub normal: Option<DynamicImage>,
    pub ao: Option<DynamicImage>,
    pub curvature: Option<DynamicImage>,
    pub height: Option<DynamicImage>,
    pub custom_channels: HashMap<String, DynamicImage>,
    pub weights: ChannelWeights,
    pub blend_mode: BlendMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelWeights {
    pub alpha: f32,
    pub normal: f32,
    pub ao: f32,
    pub curvature: f32,
    pub height: f32,
    pub custom: HashMap<String, f32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BlendMode {
    Multiply,
    Add,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
}

impl Default for ChannelWeights {
    fn default() -> Self {
        Self {
            alpha: 1.0,
            normal: 0.7,
            ao: 0.5,
            curvature: 0.3,
            height: 0.4,
            custom: HashMap::new(),
        }
    }
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::Multiply
    }
}

impl MultiChannelInput {
    pub fn new() -> Self {
        Self {
            alpha: None,
            normal: None,
            ao: None,
            curvature: None,
            height: None,
            custom_channels: HashMap::new(),
            weights: ChannelWeights::default(),
            blend_mode: BlendMode::default(),
        }
    }
    
    pub fn from_alpha(image: DynamicImage) -> Self {
        let mut input = Self::new();
        input.alpha = Some(image);
        input
    }
    
    pub fn with_alpha(mut self, image: DynamicImage) -> Self {
        self.alpha = Some(image);
        self
    }
    
    pub fn load_alpha<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SDFError> {
        let image = image::open(path)?;
        self.alpha = Some(image);
        Ok(())
    }
    
    pub fn load_normal<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SDFError> {
        let image = image::open(path)?;
        self.normal = Some(image);
        Ok(())
    }
    
    pub fn load_ao<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SDFError> {
        let image = image::open(path)?;
        self.ao = Some(image);
        Ok(())
    }
    
    pub fn load_curvature<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SDFError> {
        let image = image::open(path)?;
        self.curvature = Some(image);
        Ok(())
    }
    
    pub fn validate(&self) -> Result<(), SDFError> {
        // Check that at least one channel exists
        if self.alpha.is_none() 
            && self.normal.is_none() 
            && self.ao.is_none() 
            && self.curvature.is_none() 
            && self.height.is_none() 
            && self.custom_channels.is_empty() {
            return Err(SDFError::ValidationError {
                details: "At least one input channel is required".to_string(),
            });
        }
        
        // Check dimensions match
        let reference_dims = self.get_reference_dimensions()?;
        
        for (_name, channel) in self.iter_channels() {
            let dims = (channel.width(), channel.height());
            if dims != reference_dims {
                return Err(SDFError::DimensionMismatch {
                    expected: reference_dims.0,
                    expected_h: reference_dims.1,
                    actual_w: dims.0,
                    actual_h: dims.1,
                });
            }
        }
        
        Ok(())
    }
    
    pub fn dimensions(&self) -> Result<(u32, u32), SDFError> {
        self.get_reference_dimensions()
    }
    
    pub fn get_reference_dimensions(&self) -> Result<(u32, u32), SDFError> {
        // Use the first available channel as reference
        if let Some(img) = &self.alpha {
            return Ok((img.width(), img.height()));
        }
        if let Some(img) = &self.normal {
            return Ok((img.width(), img.height()));
        }
        if let Some(img) = &self.ao {
            return Ok((img.width(), img.height()));
        }
        if let Some(img) = &self.curvature {
            return Ok((img.width(), img.height()));
        }
        if let Some(img) = &self.height {
            return Ok((img.width(), img.height()));
        }
        if let Some((_, img)) = self.custom_channels.iter().next() {
            return Ok((img.width(), img.height()));
        }
        
        Err(SDFError::ValidationError {
            details: "No input channels available".to_string(),
        })
    }
    
    pub fn get_primary_channel(&self) -> Result<&DynamicImage, SDFError> {
        // Priority: alpha > normal > ao > curvature > height > custom
        if let Some(img) = &self.alpha {
            return Ok(img);
        }
        if let Some(img) = &self.normal {
            return Ok(img);
        }
        if let Some(img) = &self.ao {
            return Ok(img);
        }
        if let Some(img) = &self.curvature {
            return Ok(img);
        }
        if let Some(img) = &self.height {
            return Ok(img);
        }
        if let Some((_, img)) = self.custom_channels.iter().next() {
            return Ok(img);
        }
        
        Err(SDFError::ValidationError {
            details: "No input channels available".to_string(),
        })
    }
    
    pub fn has_alpha(&self) -> bool {
        self.alpha.is_some()
    }
    
    pub fn has_normal(&self) -> bool {
        self.normal.is_some()
    }
    
    pub fn has_ao(&self) -> bool {
        self.ao.is_some()
    }
    
    pub fn has_curvature(&self) -> bool {
        self.curvature.is_some()
    }
    
    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }
    
    pub fn iter_channels(&self) -> impl Iterator<Item = (&str, &DynamicImage)> {
        let mut channels = Vec::new();
        
        if let Some(img) = &self.alpha {
            channels.push(("alpha", img));
        }
        if let Some(img) = &self.normal {
            channels.push(("normal", img));
        }
        if let Some(img) = &self.ao {
            channels.push(("ao", img));
        }
        if let Some(img) = &self.curvature {
            channels.push(("curvature", img));
        }
        if let Some(img) = &self.height {
            channels.push(("height", img));
        }
        
        // Add custom channels
        for (name, img) in &self.custom_channels {
            channels.push((name.as_str(), img));
        }
        
        channels.into_iter()
    }
    
    pub fn auto_detect_channels<P: AsRef<Path>>(
        directory: P,
        basename: &str,
    ) -> Result<Self, SDFError> {
        let mut input = Self::new();
        let dir = directory.as_ref();
        
        // Common patterns for channel detection
        let patterns = [
            ("alpha", vec![
                format!("{}_diffuse", basename),
                format!("{}_alpha", basename),
                format!("{}_mask", basename),
            ]),
            ("normal", vec![
                format!("{}_normal", basename),
                format!("{}_norm", basename),
                format!("{}_n", basename),
            ]),
            ("ao", vec![
                format!("{}_ao", basename),
                format!("{}_ambient", basename),
                format!("{}_occlusion", basename),
            ]),
            ("curvature", vec![
                format!("{}_curvature", basename),
                format!("{}_curve", basename),
                format!("{}_c", basename),
            ]),
        ];
        
        let extensions = ["png", "jpg", "jpeg", "tga", "bmp"];
        
        for (channel_type, pattern_list) in &patterns {
            for pattern in pattern_list {
                for ext in &extensions {
                    let filename = format!("{}.{}", pattern, ext);
                    let filepath = dir.join(&filename);
                    
                    if filepath.exists() {
                        match *channel_type {
                            "alpha" => input.load_alpha(&filepath)?,
                            "normal" => input.load_normal(&filepath)?,
                            "ao" => input.load_ao(&filepath)?,
                            "curvature" => input.load_curvature(&filepath)?,
                            _ => {}
                        }
                        break;
                    }
                }
            }
        }
        
        Ok(input)
    }
}

impl Default for MultiChannelInput {
    fn default() -> Self {
        Self::new()
    }
}