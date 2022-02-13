// Necessary tools for getting information from the command line / external
// process
use std::env::{vars, args};
use super::fractals::Fractal;

/*------------------------------------------------------------------------
                    User Provided Information
------------------------------------------------------------------------*/
pub struct FracImgInfo<'a> {
    pub file_name: Option<&'a str>,
    pub pix_wd_ht: Option<[u32; 2]>,
    pub rng_x_y: Option<[[f32; 2]; 2]>,
    pub info: Option<Fractal>,
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

    pub fn get_width(&self) -> u32 {
        self.pix_wd_ht.unwrap()[0]
    }

    pub fn get_height(&self) -> u32 {
        self.pix_wd_ht.unwrap()[1]
    }

    pub fn get_file_name(&self) -> &'a str {
        self.file_name.unwrap()
    }

    pub fn get_x_range(&self) -> [f32; 2] {
        self.rng_x_y.unwrap()[0]
    }

    pub fn get_y_range(&self) -> [f32; 2] {
        self.rng_x_y.unwrap()[1]
    }

    pub fn get_info(&self) -> &Fractal {
        &self.info.as_ref().unwrap()
    }
}
