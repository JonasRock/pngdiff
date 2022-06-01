use clap::Parser;
extern crate image;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    image_1: std::path::PathBuf,
    #[clap(parse(from_os_str))]
    image_2: std::path::PathBuf,
    #[clap(short = 'o', long = "output", default_value = "diff.png")]
    out_path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();

    let image_1 = image::open(args.image_1.as_path()).unwrap().into_rgb8();
    let image_2 = image::open(args.image_2.as_path()).unwrap().into_rgb8();

    let mut out_buf = image::ImageBuffer::new(image_1.width(), image_1.height());

    let mut total_luma: f64 = 0.0;
    for (x, y, pixel) in out_buf.enumerate_pixels_mut() {
        let mut color: [u8; 3] = [1, 1, 1];
        color[0] = (image_1.get_pixel(x, y).0[0]).abs_diff(image_2.get_pixel(x, y).0[0]);
        color[1] = (image_1.get_pixel(x, y).0[1]).abs_diff(image_2.get_pixel(x, y).0[1]);
        color[2] = (image_1.get_pixel(x, y).0[2]).abs_diff(image_2.get_pixel(x, y).0[2]);
        let luma: u8 = (0.299 * color[0] as f32 + 0.587 * color[1] as f32 + 0.114 * color[2] as f32) as u8;
        *pixel = image::Luma([luma]);
        total_luma = total_luma + luma as f64;
    }
    
    total_luma = ((total_luma as f64) / 255.0) / (image_1.width() * image_1.height()) as f64;
    out_buf.save_with_format(args.out_path, image::ImageFormat::Png).unwrap();
    print!("{}\n", total_luma);
}