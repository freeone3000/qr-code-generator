use std::env;
use qr_code::QrCode;
use arboard::{Clipboard, ImageData};

fn conv_qr_to_clipboard(qr: &QrCode, target_width: u16) -> ImageData<'static> {
    let bmp = qr.to_bmp(); // convert to BMP first to preserve quiet zones
    let src_width = bmp.width();
    let src_height = bmp.height();

    let scale = target_width / src_width;
    let width = src_width * scale;
    let height = src_height * scale;

    // Clipboard needs it row-major.
    // [255, 0, 0, 255, 0, 255, 0, 255] is two pixels: red then green.
    // width metadata is used to determine "stride".
    let mut image_pixels = Vec::<u8>::with_capacity((src_width * src_height) as usize);

    for y in 0..src_height {
        for _ in 0..scale { // horizontal scaling
            for x in 0..src_width {
                let value = match bmp.get(x, y) {
                    true => 255,
                    false => 0,
                };
                for _ in 0..scale { // vertical scaling
                    image_pixels.push(value); // R
                    image_pixels.push(value); // G
                    image_pixels.push(value); // B
                    image_pixels.push(255);   // A
                }
            }
        }
    }

    ImageData {
        width: width as usize,
        height: height as usize,
        bytes: std::borrow::Cow::Owned(image_pixels),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let possible_text = args.get(1);
    let bytes = match possible_text {
        Some(cmd) => cmd,
        None => {
            eprintln!("Usage: {} <text to encode>", args[0]);
            std::process::exit(1);
        }
    }.as_bytes();

    let mut clipboard = Clipboard::new()?;

    let qr_code = QrCode::new(bytes)?;
    clipboard.set_image(conv_qr_to_clipboard(&qr_code, 800))?;

    Ok(())
}