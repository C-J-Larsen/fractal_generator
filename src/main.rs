mod my_complex;
mod fractals;
mod graphics;
mod comm_line_parsing;

// Import Fractal to test the behavior of Fractal::Newton
use my_complex::{MyComplex, BasicOps};
use fractals::{Fractal, FracVal, NEWTON_ITER};
use graphics::{color_mapping, bmp_img_maker};

use comm_line_parsing::FracImgInfo;

fn main() {
    let roots: Vec<MyComplex<f32>> = vec![MyComplex::new(1.0, 3.0),
        MyComplex::new(4.0, -0.4), MyComplex::new(0.5, 1.0)];
    let mut x: usize;

    println!("roots: {:?}", roots);

    let cplx_in: MyComplex<f32> = MyComplex::new(0.5, 1.1);
    println!("cplx_in: {:?}", cplx_in);

    match Fractal::Newton(roots).complex_to_frac_val(cplx_in) {
       FracVal::Newton(num) => x = num,
       _ => panic!("The fractal value was not a Newton value.")
    }

    println!("x: {}", x);
}
