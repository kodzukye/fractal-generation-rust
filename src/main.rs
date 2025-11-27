use image::{ImageBuffer, Rgba};
use num_complex::Complex;
use gif::{Encoder, Frame, Repeat};
use std::fs::File;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const MAX_ITERATIONS: u32 = 256;
const NUM_FRAMES: u32 = 120;
const ZOOM_FACTOR: f64 = 1.1;

const ZOOM_CENTER_X: f64 = -0.7469;
const ZOOM_CENTER_Y: f64 = 0.1102;

fn mandelbrot_iterations(c: Complex<f64>, max_iter: u32) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    for n in 0..max_iter {
        if z.norm_sqr() > 4.0 {
            return n;
        }
        z = z * z + c;
    }
    max_iter
}

fn render_frame(width: u32, height: u32, center_x: f64, center_y: f64, zoom: f64) -> Vec<u8> {
    let mut pixels = vec![0u8; (width * height) as usize];
    
    let view_size = 4.0 / zoom;
    let x_min = center_x - view_size / 2.0;
    let y_min = center_y - view_size / 2.0;
    let x_max = center_x + view_size / 2.0;
    let y_max = center_y + view_size / 2.0;
    
    for py in 0..height {
        for px in 0..width {
            let x = x_min + (px as f64 / width as f64) * (x_max - x_min);
            let y = y_min + (py as f64 / height as f64) * (y_max - y_min);
            let c = Complex::new(x, y);
            
            let iterations = mandelbrot_iterations(c, MAX_ITERATIONS);
            let color_index = (iterations as u8) % 255;
            
            pixels[(py * width + px) as usize] = color_index;
        }
    }
    
    pixels
}

// Create a colorful palette (256 colors)
fn create_palette() -> Vec<u8> {
    let mut palette = vec![0u8; 768]; // 256 colors * 3 (RGB)
    
    for i in 0..255 {
        let idx = i as usize * 3;
        // Create a smooth color gradient
        palette[idx] = ((i as f64 * 1.5).sin() * 127.0 + 128.0) as u8;      // R
        palette[idx + 1] = ((i as f64 * 2.0).sin() * 127.0 + 128.0) as u8;  // G
        palette[idx + 2] = ((i as f64 * 0.5).sin() * 127.0 + 128.0) as u8;  // B
    }
    
    palette
}

fn main() {
    println!("Generating infinite zoom Mandelbrot GIF...");
    
    let file = File::create("mandelbrot_zoom.gif").expect("Failed to create GIF");
    let mut encoder = Encoder::new(file, WIDTH as u16, HEIGHT as u16, &create_palette()).expect("Failed to create encoder");
    encoder.set_repeat(Repeat::Infinite).expect("Failed to set repeat");
    
    let mut zoom = 1.0;
    
    for frame_num in 0..NUM_FRAMES {
        println!("Rendering frame {}/{}", frame_num + 1, NUM_FRAMES);
        
        // Render pixels as 8-bit indexed color
        let pixels = render_frame(WIDTH, HEIGHT, ZOOM_CENTER_X, ZOOM_CENTER_Y, zoom);
        
        // Create GIF frame
        let mut frame = Frame::default();
        frame.width = WIDTH as u16;
        frame.height = HEIGHT as u16;
        frame.delay = 5; // 50ms per frame
        frame.buffer = std::borrow::Cow::Owned(pixels);
        
        encoder.write_frame(&frame).expect("Failed to write frame");
        
        zoom *= ZOOM_FACTOR;
    }
    
    println!("✨ GIF saved as mandelbrot_zoom.gif!");
}
