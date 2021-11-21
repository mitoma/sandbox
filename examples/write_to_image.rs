use image::{ImageFormat, Rgb, RgbImage};
use nenobi::{
    Back, Bounce, Circ, Cubec, EasingFunction, Elastic, Expo, Liner, Quad, Quart, Quint, Sin,
};

const IMAGE_SIZE_WIDTH: u32 = 256;
const IMAGE_SIZE_HEIGHT: u32 = 128;
const IMAGE_SIZE_HEIGHT_MARGIN: u32 = IMAGE_SIZE_HEIGHT;

fn main() {
    write_image(Liner {}, "liner");
    write_image(Sin {}, "sin-in");
    write_image(Quad {}, "quad-in");
    write_image(Cubec {}, "cubec-in");
    write_image(Quart {}, "quart-in");
    write_image(Quint {}, "quint-in");
    write_image(Expo {}, "expo-in");
    write_image(Circ {}, "circ-in");
    write_image(Back {}, "back-in");
    write_image(Elastic {}, "elastic-in");
    write_image(Bounce {}, "bounce-in");
}

fn write_image(f: impl EasingFunction<f64>, filename: &str) {
    let mut image = RgbImage::new(
        IMAGE_SIZE_WIDTH,
        IMAGE_SIZE_HEIGHT + IMAGE_SIZE_HEIGHT_MARGIN,
    );
    image.fill(255);

    for i in 0..IMAGE_SIZE_WIDTH {
        let result = 1.0 - f.s_value(i as f64 / (IMAGE_SIZE_WIDTH - 1) as f64);

        println!(
            "x:\t{:>05.5}, y:\t{:>05.5}",
            i as f64 / (IMAGE_SIZE_WIDTH - 1) as f64,
            (result * (IMAGE_SIZE_HEIGHT - 1) as f64)
        );
        image.put_pixel(
            i,
            (result * (IMAGE_SIZE_HEIGHT - 1) as f64).round() as u32 + IMAGE_SIZE_HEIGHT_MARGIN / 2,
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
