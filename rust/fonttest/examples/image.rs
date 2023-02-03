use anyhow::{Context, Result};
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use std::fmt::Write;
use ttf_parser::{Face, OutlineBuilder, Rect};

const IMAGE_SIZE_WIDTH: u32 = 256;
const IMAGE_SIZE_HEIGHT: u32 = 256;

const FONT_DATA: &[u8] = include_bytes!("../src/font/HackGenConsole-Regular.ttf");

struct ImageBuilder {
    rect: Rect,
    data: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl ImageBuilder {
    fn new(rect: Rect) -> Self {
        let mut image = RgbImage::new(IMAGE_SIZE_WIDTH + 1, IMAGE_SIZE_HEIGHT + 1);
        image.fill(255);
        Self { rect, data: image }
    }

    fn convert(&self, xy: (f32, f32)) -> (u32, u32) {
        let (mut x, mut y) = xy;
        x -= self.rect.x_min as f32;
        y -= self.rect.y_min as f32;
        (
            ((x / self.rect.width() as f32) * IMAGE_SIZE_WIDTH as f32) as u32,
            ((1. - (y / self.rect.height() as f32)) * IMAGE_SIZE_HEIGHT as f32) as u32,
        )
    }
}

impl OutlineBuilder for ImageBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        let (x, y) = self.convert((x, y));
        self.data.put_pixel(x, y, Rgb([0, 0, 0]))
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let (x, y) = self.convert((x, y));
        self.data.put_pixel(x, y, Rgb([0, 0, 0]))
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (x, y) = self.convert((x, y));
        self.data.put_pixel(x, y, Rgb([0, 0, 0]));
        let (x1, y1) = self.convert((x1, y1));
        self.data.put_pixel(x1, y1, Rgb([255, 0, 0]))
    }

    fn curve_to(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _x: f32, _y: f32) {
        todo!()
    }

    fn close(&mut self) {
        self.data
            .save_with_format(
                format!("fonttest/examples/images/{}.png", "font"),
                ImageFormat::Png,
            )
            .unwrap();
    }
}

struct Builder(String);

impl OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        writeln!(&mut self.0, "M {x} {y} ").unwrap();
    }

    fn line_to(&mut self, x: f32, y: f32) {
        writeln!(&mut self.0, "L {x} {y} ").unwrap();
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        writeln!(&mut self.0, "Q {x1} {y1} {x} {y} ").unwrap();
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        writeln!(&mut self.0, "C {x1} {y1} {x2} {y2} {x} {y} ").unwrap();
    }

    fn close(&mut self) {
        writeln!(&mut self.0, "Z ").unwrap();
    }
}

fn main() -> Result<()> {
    write_font()?;
    Ok(())
}

fn write_font() -> Result<()> {
    let c = 'あ';
    let face = Face::parse(FONT_DATA, 0)?;
    let glyph_id = face.glyph_index(c).with_context(|| "hello")?;
    let mut builder = Builder(String::new());

    let rect = face
        .outline_glyph(glyph_id, &mut builder)
        .with_context(|| "outlyne_glyph")?;

    let bounding_box = face.global_bounding_box();
    let mut image_builder = ImageBuilder::new(bounding_box);
    let _rect = face
        .outline_glyph(glyph_id, &mut image_builder)
        .with_context(|| "outlyne_glyph")?;

    println!("rect:{rect:?}");
    println!("{}", builder.0);
    Ok(())
}
