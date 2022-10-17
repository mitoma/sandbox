use anyhow::{Context, Result};
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use std::ops::{Add, Div, Mul, Sub};
use ttf_parser::{Face, OutlineBuilder, Rect};

const IMAGE_SIZE_WIDTH: u32 = 256;
const IMAGE_SIZE_HEIGHT: u32 = 256;

const FONT_DATA: &[u8] = include_bytes!("../src/font/HackGenConsole-Regular.ttf");

pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Point {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

struct ImageBuilder {
    rect: Rect,
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
    current: (f32, f32),
    polygons: Vec<[(f32, f32); 3]>,
}

impl ImageBuilder {
    fn new(rect: Rect) -> Self {
        let mut image = RgbImage::new(IMAGE_SIZE_WIDTH + 1, IMAGE_SIZE_HEIGHT + 1);
        image.fill(255);
        Self {
            rect,
            image,
            current: (0., 0.),
            polygons: Vec::new(),
        }
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
        self.current = (x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.polygons.push([(0., 0.), self.current, (x, y)]);
        self.current = (x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.polygons.push([(0., 0.), self.current, (x, y)]);

        // TODO ここにベジエ用三角も追加する

        self.current = (x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        todo!()
    }

    fn close(&mut self) {
        self.image
            .save_with_format(
                format!("fonttest/examples/images/{}.png", "write-font"),
                ImageFormat::Png,
            )
            .unwrap();
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

    let bounding_box = face.global_bounding_box();
    let mut image_builder = ImageBuilder::new(bounding_box);
    let _rect = face
        .outline_glyph(glyph_id, &mut image_builder)
        .with_context(|| "outlyne_glyph")?;

    Ok(())
}
