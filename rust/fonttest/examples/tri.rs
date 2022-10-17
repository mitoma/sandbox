use anyhow::Result;
use image::{ImageFormat, Rgb, RgbImage};

const IMAGE_SIZE_WIDTH: u32 = 200;
const IMAGE_SIZE_HEIGHT: u32 = 200;

fn main() -> Result<()> {
    let mut image = RgbImage::new(IMAGE_SIZE_WIDTH + 1, IMAGE_SIZE_HEIGHT + 1);
    image.fill(255);

    let p1 = (100, 20);
    let p2 = (20, 180);
    let p3 = (180, 180);

    for s in 0..1000 {
        for t in 0..1000 {
            let sf = s as f32 / 1000.0;
            let tf = t as f32 / 1000.0;
            let rf = 1.0 - sf - tf;
            if rf < 0.0 {
                continue;
            }
            //println!("sf={}, tf={}, rf={}", sf, tf, rf);
            let p1d = (p1.0 as f32 * sf, p1.1 as f32 * sf);
            let p2d = (p2.0 as f32 * tf, p2.1 as f32 * tf);
            let p3d = (p3.0 as f32 * rf, p3.1 as f32 * rf);
            let p = (p1d.0 + p2d.0 + p3d.0, p1d.1 + p2d.1 + p3d.1);
            //println!("{:?}", p);

            let col = if (sf / 2.0 + tf).powi(2) < tf {
                Rgb([0, 0, 0])
            } else {
                Rgb([255, 0, 0])
            };

            image.put_pixel(p.0 as u32, p.1 as u32, col);
        }
    }

    //    image.put_pixel(10, 10, Rgb([0, 0, 0]));
    image.save_with_format(
        format!("fonttest/examples/images/{}.png", "tri"),
        ImageFormat::Png,
    )?;
    Ok(())
}
