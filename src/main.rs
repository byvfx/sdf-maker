use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod core;
mod cli;
mod gui;
mod batch;

/// A Rust program that converts grayscale images into 2D Signed Distance Fields (SDFs)
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Launch GUI mode if no command is specified
    #[arg(long, global = true)]
    gui: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Process a single image file
    Process {
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        
        /// Output SDF file
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,
        
        /// Alpha/mask channel input
        #[arg(long)]
        alpha: Option<PathBuf>,
        
        /// Normal map input
        #[arg(long)]
        normal: Option<PathBuf>,
        
        /// Ambient occlusion input
        #[arg(long)]
        ao: Option<PathBuf>,
        
        /// Curvature map input
        #[arg(long)]
        curvature: Option<PathBuf>,
        
        /// Processing method
        #[arg(short, long, default_value = "jfa")]
        method: String,
        
        /// Threshold value for edge detection
        #[arg(short, long, default_value = "128")]
        threshold: u8,
        
        /// Maximum distance to calculate
        #[arg(long, default_value = "32")]
        max_distance: f32,
    },
    
    /// Process multiple images in batch mode
    Batch {
        /// Input directory
        #[arg(value_name = "DIRECTORY")]
        directory: PathBuf,
        
        /// Output directory
        #[arg(short, long, value_name = "OUTPUT")]
        output: PathBuf,
        
        /// Process recursively
        #[arg(short, long)]
        recursive: bool,
        
        /// Configuration file
        #[arg(short, long)]
        config: Option<PathBuf>,
        
        /// Number of parallel jobs
        #[arg(short, long)]
        jobs: Option<usize>,
        
        /// Show progress bar
        #[arg(short, long)]
        progress: bool,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Process { 
            input, 
            output, 
            alpha,
            normal,
            ao,
            curvature,
            method, 
            threshold, 
            max_distance 
        }) => {
            // Process single file
            cli::process_single(
                input,
                output,
                alpha,
                normal,
                ao,
                curvature,
                &method,
                threshold,
                max_distance
            )?;
        }
        Some(Commands::Batch { 
            directory, 
            output, 
            recursive, 
            config,
            jobs,
            progress 
        }) => {
            // Process batch
            batch::process_batch(
                directory,
                output,
                recursive,
                config,
                jobs,
                progress
            )?;
        }
        None => {
            // Launch GUI
            gui::launch_gui()?;
        }
    }
    
    Ok(())
}