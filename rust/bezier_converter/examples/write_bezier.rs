use anyhow::Context;
use bezier_converter::{CubicBezier, QuadraticBezier};
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};

const IMAGE_SIZE_WIDTH: u32 = 256;
const IMAGE_SIZE_HEIGHT: u32 = 256;

fn main() -> anyhow::Result<()> {
    quadratic()?;
    qubic()?;
    devide()?;
    Ok(())
}

fn quadratic() -> anyhow::Result<()> {
    let mut image = RgbImage::new(IMAGE_SIZE_WIDTH, IMAGE_SIZE_HEIGHT);
    image.fill(255);

    let qb = QuadraticBezier {
        x0: 10.0,
        y0: 10.0,
        x1: 240.0,
        y1: 240.0,
        cx0: 10.0,
        cy0: 180.0,
    };

    draw_q(&qb, &mut image)?;

    image
        .save_with_format("quadratic-bezier.png", ImageFormat::Png)
        .unwrap();
    Ok(())
}

fn draw_q(qb: &QuadraticBezier, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) -> anyhow::Result<()> {
    for i in 0..IMAGE_SIZE_WIDTH {
        let t = i as f32 / IMAGE_SIZE_WIDTH as f32;
        println!("write: t={t}");

        let point = qb.calc_point(t).with_context(|| "unwrap")?;
        image.put_pixel(point.x as u32, point.y as u32, Rgb([50, 50, 50]));
    }

    image.put_pixel(qb.x0 as u32, qb.y0 as u32, Rgb([250, 0, 0]));
    image.put_pixel(qb.x1 as u32, qb.y1 as u32, Rgb([250, 0, 0]));
    image.put_pixel(qb.cx0 as u32, qb.cy0 as u32, Rgb([0, 0, 250]));
    Ok(())
}

fn draw_c(cb: &CubicBezier, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) -> anyhow::Result<()> {
    for i in 0..IMAGE_SIZE_WIDTH {
        let t = i as f32 / IMAGE_SIZE_WIDTH as f32;
        println!("write: t={t}");

        let point = cb.calc_point(t).with_context(|| "unwrap")?;
        image.put_pixel(point.x as u32, point.y as u32, Rgb([50, 50, 50]));
    }

    image.put_pixel(cb.x0 as u32, cb.y0 as u32, Rgb([250, 0, 0]));
    image.put_pixel(cb.x1 as u32, cb.y1 as u32, Rgb([250, 0, 0]));
    image.put_pixel(cb.cx0 as u32, cb.cy0 as u32, Rgb([0, 0, 250]));
    image.put_pixel(cb.cx1 as u32, cb.cy1 as u32, Rgb([0, 0, 250]));
    Ok(())
}

fn qubic() -> anyhow::Result<()> {
    let mut image = RgbImage::new(IMAGE_SIZE_WIDTH, IMAGE_SIZE_HEIGHT);
    image.fill(255);

    let cb = CubicBezier {
        x0: 10.0,
        y0: 10.0,
        x1: 240.0,
        y1: 240.0,
        cx0: 10.0,
        cy0: 180.0,
        cx1: 180.0,
        cy1: 110.0,
    };

    draw_c(&cb, &mut image)?;

    image
        .save_with_format("qubic-bezier.png", ImageFormat::Png)
        .unwrap();
    Ok(())
}

fn devide() -> anyhow::Result<()> {
    let mut image = RgbImage::new(IMAGE_SIZE_WIDTH, IMAGE_SIZE_HEIGHT);
    image.fill(255);

    let cb = CubicBezier {
        x0: 10.0,
        y0: 10.0,
        x1: 240.0,
        y1: 240.0,
        cx0: 10.0,
        cy0: 180.0,
        cx1: 180.0,
        cy1: 10.0,
    };

    draw_c(&cb, &mut image)?;

    let qbs = cb.to_quadratic();

    for qb in qbs.iter() {
        draw_q(qb, &mut image)?;
    }

    image
        .save_with_format("devide-bezier.png", ImageFormat::Png)
        .unwrap();
    Ok(())
}
