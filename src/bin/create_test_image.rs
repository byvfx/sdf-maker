use image::{GrayImage, Luma};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple test image: white circle on black background
    let size = 256;
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
    
    std::fs::create_dir_all("assets/test_images")?;
    img.save("assets/test_images/test_circle.png")?;
    
    println!("Created test image: assets/test_images/test_circle.png");
    
    // Create a simple square test image
    let mut square_img = GrayImage::new(size, size);
    let square_size = size / 3;
    let square_start = (size - square_size) / 2;
    
    for y in 0..size {
        for x in 0..size {
            let value = if x >= square_start && x < square_start + square_size &&
                          y >= square_start && y < square_start + square_size {
                255
            } else {
                0
            };
            square_img.put_pixel(x, y, Luma([value]));
        }
    }
    
    square_img.save("assets/test_images/test_square.png")?;
    println!("Created test image: assets/test_images/test_square.png");
    
    Ok(())
}