use image::{ImageFormat, Rgb, RgbImage};
use nenobi::{
    functions::{
        back_in, back_in_out, back_out, bounce_in, bounce_in_out, bounce_out, circ_in, circ_in_out,
        circ_out, cubic_in, cubic_in_out, cubic_out, elastic_in, elastic_in_out, elastic_out,
        expo_in, expo_in_out, expo_out, liner, quad_in, quad_in_out, quad_out, quart_in,
        quart_in_out, quart_out, quint_in, quint_in_out, quint_out, sin_in, sin_in_out, sin_out,
    },
    EasingValue, Gain,
};

const IMAGE_SIZE_WIDTH: u32 = 256;
const IMAGE_SIZE_HEIGHT: u32 = 128;
const IMAGE_SIZE_HEIGHT_MARGIN: u32 = IMAGE_SIZE_HEIGHT;

fn main() {
    let mut value = EasingValue::new(0.0, 0);
    value.add(Gain::new(0.5, 10, 100, &bounce_in_out));
    value.add(Gain::new(0.4, 20, 30, &sin_in));
    value.add(Gain::new(-0.3, 100, 100, &quad_in_out));
    write_image(&value, "easing_func")
}

fn write_image(gain: &EasingValue<f64>, filename: &str) {
    let mut image = RgbImage::new(
        IMAGE_SIZE_WIDTH,
        IMAGE_SIZE_HEIGHT + IMAGE_SIZE_HEIGHT_MARGIN,
    );
    image.fill(255);

    for i in 0..IMAGE_SIZE_WIDTH {
        let result = gain.current_value(i as i32);

        println!(
            "x:\t{:>05.5}, y:\t{:>05.5}",
            i as f64 / (IMAGE_SIZE_WIDTH - 1) as f64,
            (result * (IMAGE_SIZE_HEIGHT - 1) as f64)
        );
        image.put_pixel(
            i,
            ((result * (IMAGE_SIZE_HEIGHT - 1) as f64).round()
                + (IMAGE_SIZE_HEIGHT_MARGIN / 2) as f64) as u32,
            Rgb([0, 0, 0]),
        );
    }

    image
        .save_with_format(
            format!("document/images/{}.png", filename),
            ImageFormat::Png,
        )
        .unwrap();
}
