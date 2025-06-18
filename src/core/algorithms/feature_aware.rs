use crate::core::{MultiChannelInput, SDFData, SDFError, SDFAlgorithm};

/// Feature-aware SDF algorithm that uses normal maps and other channels
/// to preserve surface details in the distance field
pub struct FeatureAwareJFA {
    threshold: u8,
    max_distance: f32,
    normal_influence: f32,
    curvature_influence: f32,
}

impl FeatureAwareJFA {
    pub fn new() -> Self {
        Self {
            threshold: 128,
            max_distance: 32.0,
            normal_influence: 0.5,
            curvature_influence: 0.3,
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
    
    pub fn with_normal_influence(mut self, influence: f32) -> Self {
        self.normal_influence = influence;
        self
    }
    
    pub fn with_curvature_influence(mut self, influence: f32) -> Self {
        self.curvature_influence = influence;
        self
    }
}

impl SDFAlgorithm for FeatureAwareJFA {
    fn process(&self, input: &MultiChannelInput) -> Result<SDFData, SDFError> {
        let (width, height) = input.dimensions()?;
        let _sdf = SDFData::new(width, height, self.max_distance);
        
        // For now, fall back to basic JFA if no special channels are available
        if !input.has_normal() && !input.has_curvature() {
            let basic_jfa = super::JumpFloodingAlgorithm::new()
                .with_threshold(self.threshold)
                .with_max_distance(self.max_distance);
            return basic_jfa.process(input);
        }
        
        // TODO: Implement feature-aware processing
        // This would analyze normal maps and curvature to:
        // 1. Detect sharp edges and preserve them
        // 2. Use surface curvature to bias distance calculations
        // 3. Create multi-layer SDFs with base shape + detail
        
        // For now, use basic processing
        let basic_jfa = super::JumpFloodingAlgorithm::new()
            .with_threshold(self.threshold)
            .with_max_distance(self.max_distance);
        basic_jfa.process(input)
    }
    
    fn name(&self) -> &'static str {
        "feature-aware-jfa"
    }
}