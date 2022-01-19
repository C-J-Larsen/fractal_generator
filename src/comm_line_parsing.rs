// Necessary tools for getting information from the command line / external
// process
use std::env::{vars, args};
use super::fractals::Fractal;

/*------------------------------------------------------------------------
                    User Provided Information
------------------------------------------------------------------------*/
pub struct FracImgInfo<'a> {
    file_name: Option<&'a str>,
    pix_wd_ht: Option<[[u16; 2]; 2]>,
    rng_x_y: Option<[[f32; 2]; 2]>,
    info: Option<Fractal>,
}

impl<'a> FracImgInfo<'a> {
    pub fn new() -> FracImgInfo<'a> {
        FracImgInfo {
            file_name: None,
            pix_wd_ht: None,
            rng_x_y: None,
            info: None,
        }
    }
}
