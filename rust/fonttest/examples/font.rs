use anyhow::{Context, Result};
use fonttest::{Point, Triangle};
use image::{ImageBuffer, ImageFormat, Rgba, RgbaImage};
use ttf_parser::{Face, OutlineBuilder, Rect};

const IMAGE_SIZE_WIDTH: u32 = 256;
const IMAGE_SIZE_HEIGHT: u32 = 256;

const FONT_DATA: &[u8] = include_bytes!("../src/font/HackGenConsole-Regular.ttf");

struct ImageBuilder {
    rect: Rect,
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    current: Point,
    polygons: Vec<Triangle>,
}

impl ImageBuilder {
    fn new(rect: Rect) -> Self {
        let mut image = RgbaImage::new(IMAGE_SIZE_WIDTH + 1, IMAGE_SIZE_HEIGHT + 1);
        image.fill(255);
        Self {
            rect,
            image,
            current: Point::new(0.0, 0.0),
            polygons: Vec::new(),
        }
    }

    fn to_font(&self, xy: (f32, f32)) -> (f32, f32) {
        let mut x = xy.0;
        let mut y = xy.1;

        x = x / IMAGE_SIZE_WIDTH as f32 * self.rect.width() as f32;
        y = y / IMAGE_SIZE_HEIGHT as f32 * self.rect.height() as f32;

        x += self.rect.x_min as f32;
        y += self.rect.y_min as f32;
        (x, y)
    }

    /*
    fn to_font(&self, xy: (u32, u32)) -> (f32, f32) {
        let mut x = xy.0 as f32;
        let mut y = xy.1 as f32;

        x = x / IMAGE_SIZE_WIDTH as f32 * self.rect.width() as f32;
        y = y / IMAGE_SIZE_HEIGHT as f32 * self.rect.height() as f32;

        x += self.rect.x_min as f32;
        y += self.rect.y_min as f32;
        (x, y)
    }
     */

    fn save_font(&mut self) {
        for x in 0..IMAGE_SIZE_WIDTH {
            for y in 0..IMAGE_SIZE_HEIGHT {
                for polygon in self.polygons.iter() {
                    let mut p = *self.image.get_pixel(x, y);
                    let font_xy = self.to_font((x as f32, y as f32));
                    if polygon.in_triangle(&Point::new(font_xy.0, font_xy.1)) {
                        p = Rgba([p.0[0] - 1, p.0[1], p.0[2], p.0[3]]);
                    }
                    let font_xy = self.to_font((x as f32 + 0.5, y as f32));
                    if polygon.in_triangle(&Point::new(font_xy.0, font_xy.1)) {
                        p = Rgba([p.0[0], p.0[1] - 1, p.0[2], p.0[3]]);
                    }
                    let font_xy = self.to_font((x as f32, y as f32 + 0.5));
                    if polygon.in_triangle(&Point::new(font_xy.0, font_xy.1)) {
                        p = Rgba([p.0[0] - 1, p.0[1], p.0[2] - 1, p.0[3]]);
                    }
                    let font_xy = self.to_font((x as f32 + 0.5, y as f32 + 0.5));
                    if polygon.in_triangle(&Point::new(font_xy.0, font_xy.1)) {
                        p = Rgba([p.0[0] - 1, p.0[1], p.0[2], p.0[3] - 1]);
                    }
                    self.image.put_pixel(x, y, p);
                }
            }
        }

        for x in 0..IMAGE_SIZE_WIDTH {
            for y in 0..IMAGE_SIZE_HEIGHT {
                let p = self.image.get_pixel(x, y);

                let mut color: u8 = 255;
                if p.0[0] % 2 == 0 {
                    color -= 63
                }
                if p.0[1] % 2 == 0 {
                    color -= 63
                }
                if p.0[2] % 2 == 0 {
                    color -= 63
                }
                if p.0[3] % 2 == 0 {
                    color -= 63
                }

                if p.0[0] % 2 == 0 {
                    self.image.put_pixel(x, y, Rgba([color, color, color, 255]))
                } else {
                    self.image.put_pixel(x, y, Rgba([255, 255, 255, 255]))
                }
            }
        }

        self.image
            .save_with_format(
                format!("fonttest/examples/images/{}.png", "write-font"),
                ImageFormat::Png,
            )
            .unwrap();
    }
}

impl OutlineBuilder for ImageBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.current = Point::new(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.polygons.push(Triangle::new(
            Point::new(0.0, 0.0),
            self.current,
            Point::new(x, y),
        ));
        self.current = Point::new(x, y);
    }

    fn quad_to(&mut self, _x1: f32, _y1: f32, x: f32, y: f32) {
        self.polygons.push(Triangle::new(
            Point::new(0.0, 0.0),
            self.current,
            Point::new(x, y),
        ));
        // TODO ここにベジエ用三角も追加する
        self.current = Point::new(x, y);
    }

    fn curve_to(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _x: f32, _y: f32) {
        todo!()
    }

    fn close(&mut self) {
        println!("count of polygon: {}", self.polygons.len())
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
    image_builder.save_font();
    Ok(())
}
