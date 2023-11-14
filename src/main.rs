use clap::Parser;
extern crate image;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    image_1: std::path::PathBuf,
    #[clap(parse(from_os_str))]
    image_2: std::path::PathBuf,
    #[clap(short = 'o', long = "output")]
    out_path: Option<std::path::PathBuf>
}

fn main() {
    let args = Cli::parse();

    let image_1 = image::open(args.image_1.as_path()).unwrap().into_rgb8();
    let image_2 = image::open(args.image_2.as_path()).unwrap().into_rgb8();

    let mut out_buf = image::ImageBuffer::new(image_1.width(), image_1.height());
    let mut floats = vec![vec![0.0f32; image_1.height() as usize]; image_1.width() as usize];

    let mut max = 0.0f32;
    let mut error = 0.0f32;
    for (x, y, _pixel) in out_buf.enumerate_pixels() {
        let mut color: [f32; 3] = [0.0, 0.0, 0.0];
        color[0] = (image_1.get_pixel(x, y).0[0]).abs_diff(image_2.get_pixel(x, y).0[0]) as f32 / 255.0;
        color[1] = (image_1.get_pixel(x, y).0[1]).abs_diff(image_2.get_pixel(x, y).0[1]) as f32 / 255.0;
        color[2] = (image_1.get_pixel(x, y).0[2]).abs_diff(image_2.get_pixel(x, y).0[2]) as f32 / 255.0;

        let tmp = (color[0] as f32) * (color[0] as f32) + (color[1] as f32) * (color[1] as f32) + (color[2] as f32) * (color[2] as f32);
        floats[x as usize][y as usize] = tmp.sqrt();
        if tmp.sqrt() > max {
            max = tmp.sqrt();
        }
        error += tmp;
    }

    for (x, y, pixel) in out_buf.enumerate_pixels_mut() {
        let mut r = 0f32;
        let mut g = 0f32;
        let mut b = 0f32;

        let n = floats[x as usize][y as usize];
        let h = if max > 0.0f32 {240.0f32*(max-n)/max} else {240.0f32};

        let s = 0.9;
        let v = 0.3 + 0.6*(n/max);

        let hi = (h/60.0).floor() as u32;
        let f = (h/60.0) - hi as f32;
        let p = v*(1.0f32-s);
        let q = v*(1.0f32-s*f);
        let t = v*(1.0f32-s*(1.0f32-f));

        if hi == 0 || hi == 6 {r=v; g=t; b=p;}
        else if hi == 1 {r=q; g=v; b=p;}
        else if hi == 2 {r=p; g=v; b=t;}
        else if hi == 3 {r=p; g=q; b=v;}
        else if hi == 4 {r=t; g=p; b=v;}
        else {r=v; g=p; b=q;}

        *pixel = image::Rgb([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]);
    }
    
    error = (error / (image_1.width() as f32 * image_1.height() as f32)).sqrt();
    if args.out_path.is_some() {
        let path = args.out_path.unwrap();
        out_buf.save_with_format(path.clone(), image::ImageFormat::Png).unwrap();
        print!("{}: {}\n", path.to_str().unwrap(), error);
    } else {
        print!("{}\n", error);
    }
}
