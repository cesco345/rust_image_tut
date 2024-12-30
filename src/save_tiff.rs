use std::fs::File;
use std::time::Instant;

fn create_tiff_multipage() -> Result<(), image::ImageError> {
    println!("Starting TIFF multipage creation...");
    
    // Load multiple images
    println!("Loading source images...");
    let images = vec![
        image::open("images/page1.jpeg")?,
        image::open("images/page2.jpeg")?,
        image::open("images/page3.jpeg")?
    ];
    
    // Process all images first
    let processed_images: Vec<_> = images.iter().map(|img| {
        let rgb_img = img.to_rgb8();
        rgb_img
    }).collect();

    // Create single output file for the TIFF
    let output_path = "multipage.tiff";
    let _file = File::create(output_path)?;
    
    // Save first image as TIFF
    processed_images[0].save(output_path)?;
    
    // Log file sizes and timing
    let mut total_input_size = 0;
    
    for (index, img) in processed_images.iter().enumerate() {
        let start_time = Instant::now();
        
        println!("\nProcessing page {}:", index + 1);
        println!("Dimensions: {}x{}", img.width(), img.height());
        
        // Get input file size
        if let Ok(metadata) = std::fs::metadata(format!("images/page{}.jpeg", index + 1)) {
            total_input_size += metadata.len();
            println!("Input file size: {} bytes", metadata.len());
        }
        
        let duration = start_time.elapsed();
        println!("Page {} processing time: {:?}", index + 1, duration);
    }
    
    // Get final size
    if let Ok(metadata) = std::fs::metadata(output_path) {
        println!("\nTIFF file created!");
        println!("Total input size: {} bytes", total_input_size);
        println!("Final TIFF size: {} bytes", metadata.len());
    }
    
    Ok(())
}

fn read_tiff_multipage(path: &str) -> Result<(), image::ImageError> {
    println!("\nReading TIFF...");
    let start_time = Instant::now();
    
    let img = image::open(path)?;
    println!("TIFF dimensions: {}x{}", img.width(), img.height());
    
    let file_size = std::fs::metadata(path)?.len();
    let duration = start_time.elapsed();
    println!("TIFF file size: {} bytes", file_size);
    println!("TIFF reading time: {:?}", duration);
    
    Ok(())
}

fn main() -> Result<(), image::ImageError> {
    // Create multipage TIFF
    match create_tiff_multipage() {
        Ok(_) => println!("TIFF creation successful!"),
        Err(e) => println!("Error creating TIFF: {}", e),
    }
    
    // Read and analyze the created TIFF
    match read_tiff_multipage("multipage.tiff") {
        Ok(_) => println!("TIFF reading successful!"),
        Err(e) => println!("Error reading TIFF: {}", e),
    }
    
    Ok(())
}