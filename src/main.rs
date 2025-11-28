use gif::{Encoder, Frame, Repeat};
use std::{error::Error, fs::File};
use rayon::prelude::*;

const WIDTH: usize = 1600;
const HEIGHT: usize = 1600;
const NUM_FRAMES: u32 = 90;
const ZOOM_SPEED: f32 = 1.05;
const ITERATIONS: u32 = 11;

#[derive(Clone, Copy)]
struct Square {
    x: f32,
    y: f32,
    side: f32,
}

struct FrameBuffer {
    pixels: Vec<u8>,
}

impl FrameBuffer {
    fn new() -> Self {
        Self {
            pixels: vec![0; WIDTH * HEIGHT * 3],
        }
    }

    fn draw_at(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        if x < WIDTH && y < HEIGHT {
            let pos = (y * WIDTH + x) * 3;
            self.pixels[pos] = rgb.0;
            self.pixels[pos + 1] = rgb.1;
            self.pixels[pos + 2] = rgb.2;
        }
    }

    fn clear(&mut self) {
        self.pixels.fill(0);
    }
}

fn generate_cantor_squares(iterations: u32) -> Vec<Square> {
    let mut squares = vec![Square {
        x: 0.0,
        y: 0.0,
        side: 1.0,
    }];

    for _ in 0..iterations {
        let mut new_squares = Vec::new();
        for sq in &squares {
            let new_side = sq.side / 3.0;
            new_squares.push(Square {
                x: sq.x,
                y: sq.y,
                side: new_side,
            });
            new_squares.push(Square {
                x: sq.x + 2.0 * new_side,
                y: sq.y,
                side: new_side,
            });
            new_squares.push(Square {
                x: sq.x,
                y: sq.y + 2.0 * new_side,
                side: new_side,
            });
            new_squares.push(Square {
                x: sq.x + 2.0 * new_side,
                y: sq.y + 2.0 * new_side,
                side: new_side,
            });
        }
        squares = new_squares;
    }

    squares
}

fn render_frame_parallel(
    cantor_squares: &[Square],
    zoom: f32,
    zoom_center_x: f32,
    zoom_center_y: f32,
    zoom_size: f32,
) -> Vec<u8> {
    let viewport_size = zoom_size / zoom;
    let viewport_x = zoom_center_x - viewport_size / 2.0;
    let viewport_y = zoom_center_y - viewport_size / 2.0;

    // Créer un buffer par thread, puis les fusionner
    let buffers: Vec<Vec<u8>> = (0..WIDTH)
        .into_par_iter()
        .map(|x| {
            let mut line = vec![0u8; HEIGHT * 3];
            
            for y in 0..HEIGHT {
                // Initialiser en noir
                let pos = y * 3;
                line[pos] = 0;
                line[pos + 1] = 0;
                line[pos + 2] = 0;
            }

            // Dessiner les carrés pour cette colonne
            for sq in cantor_squares {
                if sq.x + sq.side > viewport_x && sq.x < viewport_x + viewport_size &&
                   sq.y + sq.side > viewport_y && sq.y < viewport_y + viewport_size {
                    
                    let screen_x = (sq.x - viewport_x) / viewport_size;
                    let screen_y = (sq.y - viewport_y) / viewport_size;
                    let screen_side = sq.side / viewport_size;

                    let x1 = (screen_x * WIDTH as f32) as usize;
                    let y1 = (screen_y * HEIGHT as f32) as usize;
                    let x2 = ((screen_x + screen_side) * WIDTH as f32) as usize;
                    let y2 = ((screen_y + screen_side) * HEIGHT as f32) as usize;

                    // Dessiner la colonne de ce carré
                    if x >= x1 && x < x2.min(WIDTH) {
                        for y in y1..y2.min(HEIGHT) {
                            let pos = y * 3;
                            // Remplir
                            if y >= y1 && y < y2 {
                                line[pos] = 50;
                                line[pos + 1] = 150;
                                line[pos + 2] = 255;
                            }
                            // Bordures
                            if y == y1 || y == y2 - 1 || x == x1 || x == x2 - 1 {
                                line[pos] = 255;
                                line[pos + 1] = 255;
                                line[pos + 2] = 255;
                            }
                        }
                    }
                }
            }

            line
        })
        .collect();

    // Fusionner les lignes
    let mut pixels = vec![0u8; WIDTH * HEIGHT * 3];
    for (x, line) in buffers.iter().enumerate() {
        for y in 0..HEIGHT {
            let src_pos = y * 3;
            let dst_pos = (y * WIDTH + x) * 3;
            pixels[dst_pos] = line[src_pos];
            pixels[dst_pos + 1] = line[src_pos + 1];
            pixels[dst_pos + 2] = line[src_pos + 2];
        }
    }

    pixels
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Generating Cantor Square Zoom GIF (parallelized)...");
    
    let mut image = File::create("cantor_zoom.gif")?;
    let mut encoder = Encoder::new(&mut image, WIDTH as u16, HEIGHT as u16, &[])?;
    encoder.set_repeat(Repeat::Infinite)?;

    let cantor_squares = generate_cantor_squares(ITERATIONS);
    
    let zoom_center_x = 1.0 / 3.0;
    let zoom_center_y = 1.0 / 3.0;
    let zoom_size = 1.0 / 3.0;

    let mut zoom = 1.0f32;

    for frame_num in 0..NUM_FRAMES {
        println!("Frame {}/{}", frame_num + 1, NUM_FRAMES);
        
        let pixels = render_frame_parallel(&cantor_squares, zoom, zoom_center_x, zoom_center_y, zoom_size);

        let frame = Frame::from_rgb(WIDTH as u16, HEIGHT as u16, &pixels);
        encoder.write_frame(&frame)?;

        zoom *= ZOOM_SPEED;
    }

    println!("✨ GIF saved as cantor_zoom.gif!");
    Ok(())
}
