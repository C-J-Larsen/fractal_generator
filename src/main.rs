mod my_complex;
mod fractals;
mod graphics;

use graphics::{color_mapping::FracOutput_to_Color, bmp_img_maker::BMPImg};
use my_complex::MyComplex;
use fractals::{Fractal, FractalType};

extern crate clap;
use clap::{Command, Arg, Values};

fn str_to_val<T: std::str::FromStr>(inpStr: &str) -> T
where
    T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let new_inp_str: String = inpStr.replace("~", "-");
    new_inp_str.parse::<T>().unwrap()
}

fn main() {
    // Parse the command line arguments
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("width").takes_value(true).required(true)
            .help("The width of the image in pixels")
        )
        .arg(Arg::new("height").takes_value(true).required(true)
            .help("The height of the image in pixels")
        )
        .arg(Arg::new("file-name").short('f').long("file-name")
            .default_value("fractal.bmp")
            .help("The name of the image to output.")
        )
        .arg(Arg::new("max-iters").default_value("1000")
            .help("The maximum number of iterations to run the fractal algorithms \
            before moving on.")
        )
        .arg(Arg::new("real-range").short('r').long("real-range")
            .number_of_values(2).value_names(&["start", "end"]).default_values(&["-2", "2"])
            .help("The start and end coordinates of the real axis of the image.")
        )
        .arg(Arg::new("imag-range").short('i').long("imag-range")
            .number_of_values(2).value_names(&["start", "end"]).default_values(&["-2", "2"])
            .help("The start and end coordinates of the imaginary axis of the image.")
        )
        .subcommand(Command::new("Mandelbrot")
            .about("Generates a Mandelbrot set fractal image")
        )
        .subcommand(Command::new("Julia")
            .arg(Arg::new("real").required(true)
                .help("The real component of the complex number seed of the Julia fractal.")
            )
            .arg(Arg::new("imaginary").required(true)
                .help("The imaginary component of the complex number seed of the Julia fractal.")
            )
            .about("Generates a Julia set fractal image")
        )
        .subcommand(Command::new("Newton")
            .arg(Arg::new("root").short('r').long("root").required(true)
                .multiple_occurrences(true).number_of_values(2).value_names(&["real", "imag"])
                .help("A complex number representing a polynomial root (the 'a' in the 'x-a' linear term). \
                       List as many roots as are needed for the fractal.")
            )
            .about("Generates a Newton fractal image")
        )
        .get_matches();

    /**************************************************************************
     * Pull the information needed from the command line arguments
     *************************************************************************/
    // Create the object that will handle making the BMP image
    let width:     u32  = str_to_val(matches.value_of("width").unwrap());
    let height:    u32  = str_to_val(matches.value_of("height").unwrap());
    let file_name: &str = matches.value_of("file-name").unwrap();
    let bmp_img_obj = BMPImg::new(width, height, file_name);

    /**************************************************************************
     * Make the Fractal object
     *************************************************************************/
    let frac_kind: FractalType = 
        if let (frac_name, frac_matches) = matches.subcommand().unwrap() {
            if frac_name == "Mandelbrot" {
                FractalType::Mandelbrot ()
            } else if frac_name == "Julia" {
                let real_seed: f32 = str_to_val(frac_matches.values_of("real").unwrap().next().unwrap());
                let imag_seed: f32 = str_to_val(frac_matches.values_of("imaginary").unwrap().next().unwrap());
                FractalType::Julia (MyComplex::new(real_seed, imag_seed))
            } else { // Must be a Newton fractal
                // If a Newton fractal, get the values of each of the roots.
                let mut roots: Vec<MyComplex<f32>> = Vec::new();

                'root: loop {
                    let real: f32;
                    let imag: f32;
                    match frac_matches.values_of("root").unwrap().next() {
                        Some(real_str) => real = str_to_val(real_str),
                        None => break 'root,
                    }
                    match frac_matches.values_of("root").unwrap().next() {
                        Some(imag_str) => imag = str_to_val(imag_str),
                        None => break 'root,
                    }
                    roots.push(MyComplex::new(real, imag));
                }
                FractalType::Newton (roots)
            }
        } else {
            panic!();
    };

    // Put together the fractal object
    let frac_obj: &Fractal = &Fractal::new(
        str_to_val(matches.value_of("max-iters").unwrap()),
        frac_kind,
    );

    /**************************************************************************
     * Create the fractal
     *************************************************************************/
    // Get the values that will be used in the pixel-to-complex coordinate
    // calculation.
    let width_as_f32:  f32 = width as f32;
    let height_as_f32: f32 = height as f32;

    let mut r_inputs: Values = matches.values_of("real-range").unwrap();
    let r_start: f32 = str_to_val(r_inputs.next().unwrap());
    let r_range: f32 = str_to_val::<f32>(r_inputs.next().unwrap()) - r_start;

    let mut i_inputs: Values = matches.values_of("imag-range").unwrap();
    let i_start: f32 = str_to_val(i_inputs.next().unwrap());
    let i_range: f32 = str_to_val::<f32>(i_inputs.next().unwrap()) - i_start;

    // Create the closure that colors each pixel
    let pix_pos_to_color = |pix_pos: graphics::PixPos| -> graphics::Color {
        // Pixels are colored left to right, bottom to top
        let real: f32 = r_start + (pix_pos.col as f32) / width_as_f32  * r_range;
        let imag: f32 = i_start + (pix_pos.row as f32) / height_as_f32 * i_range;
        let cmplx_num: MyComplex<f32> = MyComplex::new(real, imag);

        FracOutput_to_Color(frac_obj.complex_to_frac_output(cmplx_num))
    };

    // Create the image
    bmp_img_obj.set_header();
    bmp_img_obj.color_pixels(&pix_pos_to_color);
}
