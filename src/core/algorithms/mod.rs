pub mod brute_force;
pub mod jfa;
pub mod feature_aware;

use crate::core::{MultiChannelInput, SDFData, SDFError};

pub use brute_force::BruteForce;
pub use jfa::JumpFloodingAlgorithm;
pub use feature_aware::FeatureAwareJFA;

/// Trait for all SDF generation algorithms
pub trait SDFAlgorithm: Send + Sync {
    fn process(&self, input: &MultiChannelInput) -> Result<SDFData, SDFError>;
    fn name(&self) -> &'static str;
}

/// Factory function to create algorithm by name
pub fn create_algorithm(name: &str) -> Result<Box<dyn SDFAlgorithm>, SDFError> {
    match name.to_lowercase().as_str() {
        "brute" | "brute-force" => Ok(Box::new(BruteForce::new())),
        "jfa" | "jump-flooding" => Ok(Box::new(JumpFloodingAlgorithm::new())),
        "feature-aware" | "feature-aware-jfa" => Ok(Box::new(FeatureAwareJFA::new())),
        _ => Err(SDFError::ProcessingFailed {
            reason: format!("Unknown algorithm: {}", name),
        }),
    }
}