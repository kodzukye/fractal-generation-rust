use image::{ImageBuffer, Rgba, RgbaImage};
use std::io::{self, Write};

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
        // Draw the square
        draw_filled_rectangle(image, x, y, size, color);
        return;
    }

    let sub_size = size / 3;

    // Draw only 4 corner squares (0,0), (0,2), (2,0), (2,2)
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

fn main() {
    println!("=== Générateur de Carré de Cantor ===\n");

    // Get initial size
    print!("Taille du carré initial (pixels) [par défaut: 729]: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let size: u32 = input.trim().parse().unwrap_or(729);

    // Get iterations
    print!("Nombre d'itérations [par défaut: 4]: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let iterations: u32 = input.trim().parse().unwrap_or(4);

    // Get output filename
    print!("Nom du fichier de sortie [par défaut: cantor.png]: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let filename = match input.trim() {
        "" => "cantor.png",
        name => name,
    };

    // Create a new image buffer with white background
    let mut image = RgbaImage::from_pixel(size, size, Rgba([255, 255, 255, 255]));

    // Color: black squares
    let color = Rgba([0, 0, 0, 255]);

    // Draw the Cantor square
    println!("\nGénération du carré de Cantor...");
    draw_cantor_square(&mut image, 0, 0, size, iterations, color);

    // Save the image
    image.save(filename).expect("Erreur lors de la sauvegarde");
    println!("✓ Image sauvegardée: {}", filename);
    println!("  - Taille: {} × {} pixels", size, size);
    println!("  - Itérations: {}", iterations);
    println!("  - Carrés générés: 4^{} = {}", iterations, 4_u32.pow(iterations));
}
