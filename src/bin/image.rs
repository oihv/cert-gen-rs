use image::{ImageReader, Rgba};
use imageproc::{self, drawing};
use ab_glyph::{FontRef};
use rand::{self, random_range};

fn main() {
    let mut img = ImageReader::open("Welcome_Certificate.jpg").unwrap().decode().unwrap();

    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/OTF/CodeNewRomanNerdFont-Regular.otf")).unwrap();

    let width = img.width();
    let height = img.height();

    // let mut rng = rand::rng();
    let r = random_range(0..=255);
    let g = random_range(0..=255);
    let b = random_range(0..=255);
    drawing::draw_text_mut(&mut img, Rgba::from([r,g,b,255]), width as i32/2, height as i32/2, 160., &font, "Anjay");
    let _ = img.save("Welcome_Certificate_new.jpg");
}
