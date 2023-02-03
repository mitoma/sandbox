use image::{ImageFormat, Rgb, RgbImage};
use nenobi::functions::{
    back_in, back_in_out, back_out, bounce_in, bounce_in_out, bounce_out, circ_in, circ_in_out,
    circ_out, cubic_in, cubic_in_out, cubic_out, elastic_in, elastic_in_out, elastic_out, expo_in,
    expo_in_out, expo_out, liner, quad_in, quad_in_out, quad_out, quart_in, quart_in_out,
    quart_out, quint_in, quint_in_out, quint_out, sin_in, sin_in_out, sin_out,
};

const IMAGE_SIZE_WIDTH: u32 = 256;
const IMAGE_SIZE_HEIGHT: u32 = 128;
const IMAGE_SIZE_HEIGHT_MARGIN: u32 = IMAGE_SIZE_HEIGHT;

fn main() {
    write_image(liner, "liner");
    // in
    write_image(sin_in, "sin-in");
    write_image(quad_in, "quad-in");
    write_image(cubic_in, "cubic-in");
    write_image(quart_in, "quart-in");
    write_image(quint_in, "quint-in");
    write_image(expo_in, "expo-in");
    write_image(circ_in, "circ-in");
    write_image(back_in, "back-in");
    write_image(elastic_in, "elastic-in");
    write_image(bounce_in, "bounce-in");
    // out
    write_image(sin_out, "sin-out");
    write_image(quad_out, "quad-out");
    write_image(cubic_out, "cubic-out");
    write_image(quart_out, "quart-out");
    write_image(quint_out, "quint-out");
    write_image(expo_out, "expo-out");
    write_image(circ_out, "circ-out");
    write_image(back_out, "back-out");
    write_image(elastic_out, "elastic-out");
    write_image(bounce_out, "bounce-out");
    // in out
    write_image(sin_in_out, "sin-in-out");
    write_image(quad_in_out, "quad-in-out");
    write_image(cubic_in_out, "cubic-in-out");
    write_image(quart_in_out, "quart-in-out");
    write_image(quint_in_out, "quint-in-out");
    write_image(expo_in_out, "expo-in-out");
    write_image(circ_in_out, "circ-in-out");
    write_image(back_in_out, "back-in-out");
    write_image(elastic_in_out, "elastic-in-out");
    write_image(bounce_in_out, "bounce-in-out");
}

fn write_image(f: fn(f64) -> f64, filename: &str) {
    let mut image = RgbImage::new(
        IMAGE_SIZE_WIDTH,
        IMAGE_SIZE_HEIGHT + IMAGE_SIZE_HEIGHT_MARGIN,
    );
    image.fill(255);

    for i in 0..IMAGE_SIZE_WIDTH {
        let result = 1.0 - f(i as f64 / (IMAGE_SIZE_WIDTH - 1) as f64);

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
            format!("document/images/{filename}.png"),
            ImageFormat::Png,
        )
        .unwrap();
}
