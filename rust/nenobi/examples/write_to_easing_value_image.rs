use image::{ImageFormat, Rgb, RgbImage};
use nenobi::{
    EasingValue, Gain,
    functions::{back_in_out, bounce_in_out, quad_in_out, sin_in},
};

const IMAGE_SIZE_WIDTH: u32 = 256;
const IMAGE_SIZE_HEIGHT: u32 = 128;
const IMAGE_SIZE_HEIGHT_MARGIN: u32 = IMAGE_SIZE_HEIGHT;

fn main() {
    let mut value = EasingValue::new(0.0);
    value.add(Gain::new(0.5, 10, 100, bounce_in_out));
    value.add(Gain::new(0.4, 20, 30, sin_in));
    value.add(Gain::new(-0.3, 100, 100, quad_in_out));
    value.add(Gain::new(0.2, 125, 50, back_in_out));
    write_image(&value, "easing_func")
}

fn write_image(gain: &EasingValue<f64>, filename: &str) {
    let mut image = RgbImage::new(
        IMAGE_SIZE_WIDTH,
        IMAGE_SIZE_HEIGHT + IMAGE_SIZE_HEIGHT_MARGIN,
    );
    image.fill(255);

    for i in 0..IMAGE_SIZE_WIDTH {
        let result = gain.current_value(i as i64);

        println!(
            "x:\t{:>05.5}, y:\t{:>05.5}, result:{}",
            i as f64 / (IMAGE_SIZE_WIDTH - 1) as f64,
            (result * (IMAGE_SIZE_HEIGHT - 1) as f64),
            result
        );
        image.put_pixel(
            i,
            ((result * (IMAGE_SIZE_HEIGHT - 1) as f64).round()
                + (IMAGE_SIZE_HEIGHT_MARGIN / 2) as f64) as u32,
            Rgb([0, 0, 0]),
        );
    }

    image
        .save_with_format(format!("document/images/{filename}.png"), ImageFormat::Png)
        .unwrap();
}
