use anyhow::Result;
use fonttest::Point;
use image::{ImageFormat, Rgb, RgbImage};

const IMAGE_SIZE_WIDTH: u32 = 200;
const IMAGE_SIZE_HEIGHT: u32 = 200;

fn main() -> Result<()> {
    let mut image = RgbImage::new(IMAGE_SIZE_WIDTH + 1, IMAGE_SIZE_HEIGHT + 1);
    image.fill(255);

    let p1 = Point::new(100., 20.);
    let p2 = Point::new(20., 180.);
    let p3 = Point::new(180., 180.);

    for s in 0..1000 {
        for t in 0..1000 {
            let sf = s as f32 / 1000.0;
            let tf = t as f32 / 1000.0;
            let rf = 1.0 - sf - tf;
            if rf < 0.0 {
                continue;
            }
            let p1d = p1 * sf;
            let p2d = p2 * tf;
            let p3d = p3 * rf;
            let p = p1d + p2d + p3d;

            let col = if (sf / 2.0 + tf).powi(2) < tf {
                Rgb([0, 0, 0])
            } else {
                Rgb([255, 0, 0])
            };

            image.put_pixel(p.x as u32, p.y as u32, col);
        }
    }

    image.save_with_format(
        format!("fonttest/examples/images/{}.png", "tri"),
        ImageFormat::Png,
    )?;
    Ok(())
}
