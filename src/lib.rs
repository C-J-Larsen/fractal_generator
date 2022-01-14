/*------------------------------------------------------------------------
                            MODULES
------------------------------------------------------------------------*/
// Define all of the modules needed for main here.
mod my_complex;
mod fractals;

// Import whatever is needed from the submodules here
use my_complex::{MyComplex, BasicOps};
use fractals::Fractal;

/*------------------------------------------------------------------------

------------------------------------------------------------------------*/
struct FracImgInfo<'a> {
    file_name: Option<&'a str>,
    pix_wd_ht: Option<[[u16; 2]; 2]>,
    rng_x_y: Option<[[f32; 2]; 2]>,
    info: Option<Fractal>,
}

impl<'a> FracImgInfo<'a> {
    fn new() -> FracImgInfo<'a> {
        FracImgInfo {
            file_name: None,
            pix_wd_ht: None,
            rng_x_y: None,
            info: None,
        }
    }
}

/*------------------------------------------------------------------------
                                TESTS
------------------------------------------------------------------------*/
