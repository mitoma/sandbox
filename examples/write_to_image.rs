use image::{ImageFormat, Rgb, RgbImage};
use nenobi::{
    Back, Bounce, Circ, Cubec, EasingFunction, Elastic, Expo, InOut, Liner, Quad, Quart, Quint,
    Reverse, Sin,
};

const IMAGE_SIZE_WIDTH: u32 = 256;
const IMAGE_SIZE_HEIGHT: u32 = 128;
const IMAGE_SIZE_HEIGHT_MARGIN: u32 = IMAGE_SIZE_HEIGHT;

fn main() {
    write_image(Liner::default(), "liner");
    write_image(Sin::default(), "sin-in");
    write_image(Quad::default(), "quad-in");
    write_image(Cubec::default(), "cubec-in");
    write_image(Quart::default(), "quart-in");
    write_image(Quint::default(), "quint-in");
    write_image(Expo::default(), "expo-in");
    write_image(Circ::default(), "circ-in");
    write_image(Back::default(), "back-in");
    write_image(Reverse::new(Box::new(Back::default())), "back-out");
    write_image(Elastic::default(), "elastic-in");
    write_image(Reverse::new(Box::new(Elastic::default())), "elastic-out");
    write_image(Bounce::default(), "bounce-in");
    write_image(Reverse::new(Box::new(Bounce::default())), "bounce-out");
    write_image(
        InOut::new(Box::new(Bounce::default()), Box::new(Bounce::default())),
        "bounce-inout",
    );
}

fn write_image(f: impl EasingFunction<f64>, filename: &str) {
    let mut image = RgbImage::new(
        IMAGE_SIZE_WIDTH,
        IMAGE_SIZE_HEIGHT + IMAGE_SIZE_HEIGHT_MARGIN,
    );
    image.fill(255);

    for i in 0..IMAGE_SIZE_WIDTH {
        let result = 1.0 - f.value(i as f64 / (IMAGE_SIZE_WIDTH - 1) as f64);

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
