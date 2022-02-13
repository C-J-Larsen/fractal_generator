mod fractals;
mod graphics;
mod comm_line_parsing;

use graphics::color_mapping::FracVal_to_Color;
// Import Fractal to test the behavior of Fractal::Newton
use my_complex::{MyComplex, BasicOps};
use fractals::{Fractal, FracVal, NEWTON_ITER};

use comm_line_parsing::FracImgInfo; 

fn main() {
    let roots: Vec<MyComplex<f32>> = vec![MyComplex::new(1.0, 1.0),
        MyComplex::new(-1.0, 1.0), MyComplex::new(-1.0, -1.0),
        MyComplex::new(1.0, -1.0)];

    let info = FracImgInfo { 
        file_name: Some("test.bmp"),
        pix_wd_ht: Some([400_u32, 400_u32]),
        rng_x_y: Some([[-2.0, 2.0], [-2.0, 2.0]]),
        info: Some(Fractal::Newton(roots)),
    };

    let bmp_img_obj = graphics::bmp_img_maker::BMPImg::new(
        info.get_width(),
        info.get_height(),
        info.get_file_name()
    );
 
    let width: f32 = info.get_width() as f32;
    let height: f32 = info.get_height() as f32;
    let x_start: f32 = info.get_x_range()[0];
    let x_range: f32 = info.get_x_range()[1] - x_start;
    let y_start: f32 = info.get_y_range()[0];
    let y_range: f32 = info.get_y_range()[1] - y_start;
    let frac_obj: &Fractal = info.get_info();

    let pix_pos_to_color = |pix_pos: graphics::PixPos| -> graphics::Color {
        // Pixels are colored left to right, bottom to top
        let real: f32 = x_start + (pix_pos.col as f32) / width * x_range;
        let imag: f32 = y_start + (pix_pos.row as f32) / height * y_range;
        let cmplx_num: MyComplex<f32> = MyComplex::new(real, imag);

        FracVal_to_Color(frac_obj.complex_to_frac_val(cmplx_num))
    };

    bmp_img_obj.set_header();
    bmp_img_obj.color_pixels(&pix_pos_to_color);
}
