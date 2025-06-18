use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sdf_maker::core::{MultiChannelInput, algorithms::{BruteForce, JumpFloodingAlgorithm, SDFAlgorithm}};
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

fn bench_brute_force(c: &mut Criterion) {
    let input_64 = create_test_circle(64);
    let input_128 = create_test_circle(128);
    let input_256 = create_test_circle(256);
    
    let algorithm = BruteForce::new();
    
    c.bench_function("brute_force_64x64", |b| {
        b.iter(|| algorithm.process(black_box(&input_64)))
    });
    
    c.bench_function("brute_force_128x128", |b| {
        b.iter(|| algorithm.process(black_box(&input_128)))
    });
    
    c.bench_function("brute_force_256x256", |b| {
        b.iter(|| algorithm.process(black_box(&input_256)))
    });
}

fn bench_jump_flooding(c: &mut Criterion) {
    let input_64 = create_test_circle(64);
    let input_128 = create_test_circle(128);
    let input_256 = create_test_circle(256);
    
    let algorithm = JumpFloodingAlgorithm::new();
    
    c.bench_function("jfa_64x64", |b| {
        b.iter(|| algorithm.process(black_box(&input_64)))
    });
    
    c.bench_function("jfa_128x128", |b| {
        b.iter(|| algorithm.process(black_box(&input_128)))
    });
    
    c.bench_function("jfa_256x256", |b| {
        b.iter(|| algorithm.process(black_box(&input_256)))
    });
}

criterion_group!(benches, bench_brute_force, bench_jump_flooding);
criterion_main!(benches);