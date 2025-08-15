use watchdog::{PixelFormat, PlatformCapture};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing basic screen capture...");

    let capture = PlatformCapture::new()?;
    let frame = capture.capture_full_screen()?;

    println!(
        "Captured frame: {}x{} ({} bytes, format: {:?})",
        frame.width,
        frame.height,
        frame.data.len(),
        frame.format
    );

    // Convert to RGB (dropping alpha channel)
    let rgb_data = match frame.format {
        PixelFormat::Bgra => {
            let mut rgb_data = Vec::with_capacity(frame.width as usize * frame.height as usize * 3);

            for y in 0..frame.height {
                let row_start = y as usize * frame.stride;
                for x in 0..frame.width {
                    let pixel_offset = row_start + (x as usize * 4);
                    if pixel_offset + 3 < frame.data.len() {
                        let b = frame.data[pixel_offset];
                        let g = frame.data[pixel_offset + 1];
                        let r = frame.data[pixel_offset + 2];

                        // Convert BGR -> RGB
                        rgb_data.push(r);
                        rgb_data.push(g);
                        rgb_data.push(b);
                    }
                }
            }
            rgb_data
        }
        PixelFormat::Rgb => frame.data.clone(),
    };

    let img = image::RgbImage::from_raw(frame.width, frame.height, rgb_data)
        .ok_or("Failed to create image from frame data")?;

    let filename = format!("screen_capture_{}x{}.png", frame.width, frame.height);
    img.save(&filename)?;

    println!("Saved screenshot as: {}", filename);
    Ok(())
}
