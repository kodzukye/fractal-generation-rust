use image::{ImageBuffer, Rgba, RgbaImage};
use std::io::{self, Write};
use gif::{Frame, Encoder, Repeat};
use std::fs::File;
use std::borrow::Cow;

fn draw_cantor_square(
    image: &mut RgbaImage,
    x: u32,
    y: u32,
    size: u32,
    iterations: u32,
    color: Rgba<u8>,
) {
    if size == 0 {
        return;
    }

    if iterations == 0 {
        draw_filled_rectangle(image, x, y, size, color);
        return;
    }

    let sub_size = size / 3;

    // Draw only 4 corner squares
    for i in [0, 2].iter() {
        for j in [0, 2].iter() {
            draw_cantor_square(
                image,
                x + i * sub_size,
                y + j * sub_size,
                sub_size,
                iterations - 1,
                color,
            );
        }
    }
}

fn draw_filled_rectangle(
    image: &mut RgbaImage,
    x: u32,
    y: u32,
    size: u32,
    color: Rgba<u8>,
) {
    for i in 0..size {
        for j in 0..size {
            let px = x + i;
            let py = y + j;
            if px < image.width() && py < image.height() {
                image.put_pixel(px, py, color);
            }
        }
    }
}

fn rgba_to_indexed(image: &RgbaImage) -> Vec<u8> {
    let mut indexed = Vec::new();
    for pixel in image.pixels() {
        // Simple conversion: if pixel is black, use index 1; otherwise index 0 (white)
        if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
            indexed.push(1);
        } else {
            indexed.push(0);
        }
    }
    indexed
}

fn main() {
    println!("=== Générateur de Carré de Cantor (GIF Animé) ===\n");

    // Get initial size
    print!("Taille du carré initial (pixels) [par défaut: 486]: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let size: u32 = input.trim().parse().unwrap_or(486);

    // Get max iterations
    print!("Nombre maximum d'itérations [par défaut: 4]: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let max_iterations: u32 = input.trim().parse().unwrap_or(4);

    // Get frame delay (milliseconds)
    print!("Délai entre les frames en ms [par défaut: 500]: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let frame_delay: u16 = input.trim().parse().unwrap_or(500);

    // Get output filename
    print!("Nom du fichier GIF [par défaut: cantor_animation.gif]: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let filename = match input.trim() {
        "" => "cantor_animation.gif".to_string(),
        name => name.to_string(),
    };

    // Color palette: white (0,0,0) and black (1,0,0)
    let color_map = vec![255, 255, 255, 0, 0, 0];

    println!("\nGénération de l'animation GIF...");

    // Create GIF encoder
    let output = File::create(&filename)
        .expect("Erreur: impossible de créer le fichier GIF");
    let mut encoder = Encoder::new(output, size as u16, size as u16, &color_map)
        .expect("Erreur: impossible de créer l'encodeur GIF");
    
    encoder.set_repeat(Repeat::Infinite)
        .expect("Erreur: impossible de définir la boucle infinie");

    // Generate frames for each iteration
    for current_iter in 0..=max_iterations {
        // Create a new image for this iteration
        let mut image = RgbaImage::from_pixel(size, size, Rgba([255, 255, 255, 255]));
        let color = Rgba([0, 0, 0, 255]);

        // Draw the Cantor square at this iteration level
        draw_cantor_square(&mut image, 0, 0, size, current_iter, color);

        // Convert RGBA to indexed color (black/white only)
        let indexed = rgba_to_indexed(&image);

        // Create frame
        let mut frame = Frame::default();
        frame.width = size as u16;
        frame.height = size as u16;
        frame.delay = frame_delay / 10; // GIF delay is in units of 10ms
        frame.buffer = Cow::Borrowed(&indexed);

        encoder.write_frame(&frame)
            .expect("Erreur: impossible d'écrire la frame GIF");

        println!("✓ Itération {} générée", current_iter);
    }

    println!("\n✓ GIF animé sauvegardé: {}", filename);
    println!("  - Taille: {} × {} pixels", size, size);
    println!("  - Itérations: 0 à {}", max_iterations);
    println!("  - Délai par frame: {}ms", frame_delay);
    println!("  - Nombre total de frames: {}", max_iterations + 1);
}
