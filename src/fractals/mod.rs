mod mandelbrot_fractal;
mod julia_fractal;
mod newton_fractal;

use super::my_complex::MyComplex;

// Default the maximum iterations to 1_000
static max_iter: u32 = 1_000_u32;

// Fractals can come in three varieties: Mandelbrot fractals, Julia fractals,
// and Newton fractals.
// NOTE: For now, Fractal will only be implemented for floats.
pub enum Fractal {
    // A Mandelbrot set doesn't have any input variables; it's the same every
    // time
    Mandelbrot (),
    // A Julia set is defined by a complex constant to add in the iterative
    // algorithm
    Julia (MyComplex<f32>),
    // A Newton fractal is defined by the complex roots
    Newton (Vec<MyComplex<f32>>),
}

// The output of a Fractal algorithm comes in two varieties:
// 1) The number of iterations before the value diverges (Mandelbrot/Julia)
// 2) The closest root after `n` iterations (Newton)
pub enum FracVal {
    MandelJulia(u32),
    Newton(u32),
}

// All that needs to be implemented for a Fractal is the algorithm that
// converts a complex number to some divergence / nearest root value
impl Fractal {
    pub fn complex_to_frac_val(self, z_in: MyComplex<f32>) -> FracVal {
            let mut i: u32 = 0_u32;
        match self {
            Fractal::Mandelbrot() => {
                // The iteration for a Mandelbrot set
                // Iterate while the magniture is less than 2
                let mut z: MyComplex<f32> = MyComplex::new(0.0, 0.0);
                while z.mag_sqr() < 4.0_f32 && i < max_iter {
                    z = z*z + z_in;;
                    i += 1;
                }
                FracVal::MandelJulia(i)
            }
            Fractal::Julia(z_const) => {
                // The iteration for a Julia set (increment by z_const in the
                // iterating algorithm)
                let mut z: MyComplex<f32> = z_in;
                while z.mag_sqr() < 4.0 && i < max_iter {
                    z = z*z + z_const;
                    i += 1;
                }
                FracVal::MandelJulia(i)
            }
            Fractal::Newton(roots) => {
                // The iteration for a Newton fractal (use the roots)
                FracVal::Newton(0_u32)
            }
        }
    }
}

/*------------------------------------------------------------------------
                                TESTS
------------------------------------------------------------------------*/
#[cfg(test)]
mod tests {
    use super::*;
    use super::super::my_complex::MyComplex;
    // Check some known values on the Mandelbrot set, and make sure the
    // divergence values make sense
    #[test]
    fn test1 () {
        // cmplx_in: [MyComplex<f32>; 5] = [];
        // known_vals: [f32; 5] = [];
    }
}
