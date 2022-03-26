use super::my_complex::MyComplex;

// Fractals can come in three varieties: Mandelbrot fractals, Julia fractals,
// and Newton fractals.
// NOTE: For now, Fractal will only be implemented for floats.
pub enum FractalType {
    // A Mandelbrot set doesn't have any input variables; it's the same every
    // time
    Mandelbrot (),
    // A Julia set is defined by a complex constant to add in the iterative
    // algorithm
    Julia (MyComplex<f32>),
    // A Newton fractal is defined by the complex roots
    Newton (Vec<MyComplex<f32>>),
}

pub struct Fractal {
    max_iter: u32,
    kind: FractalType,
}

// The output of a Fractal algorithm comes in two varieties:
// 1) The number of iterations before the value diverges (Mandelbrot/Julia)
// 2) The closest root after `n` iterations (Newton)
#[derive(Clone, Copy, Debug)]
pub enum FracOutput {
    MandelJulia {iters:u32, max_iters:u32},
    Newton {closest:usize, roots:usize},
}

// All that needs to be implemented for a Fractal is the algorithm that
// converts a complex number to some divergence / nearest root value
impl Fractal {
    pub fn new(max_iter: u32, kind: FractalType) -> Self {
        Fractal {max_iter, kind}
    }

    pub fn set_max_iter(&mut self, val: u32) {
        self.max_iter = val;
    }

    pub fn set_kind(&mut self, val: FractalType) {
        self.kind = val;
    }

    pub fn complex_to_frac_output(&self, z_in: MyComplex<f32>) -> FracOutput {

        let mut iterations: u32 = 0_u32;
        match &self.kind {
            FractalType::Mandelbrot() => {
                // The iteration for a Mandelbrot set
                // Iterate while the magniture is less than 2
                let mut z: MyComplex<f32> = MyComplex::new(0.0, 0.0);
                for i in 1..=self.max_iter {
                    z = z*z + z_in;
                    iterations = i;
                    if z.mag_sqr() > 4.0_f32 { 
                        break;
                    }
                }
                FracOutput::MandelJulia{iters:iterations, max_iters:self.max_iter}
            }
            // NOTE: since self is a borrowed value (aka immutable pass by
            // reference), z_const is a reference to an enum value. To use it,
            // it must be dereferenced.
            FractalType::Julia(z_const) => {
                // The iteration for a Julia set (increment by z_const in the
                // iterating algorithm)
                let mut z: MyComplex<f32> = z_in;
                for i in 1..=self.max_iter {
                    z = z*z + *z_const;
                    iterations = i;
                    if z.mag_sqr() > 4.0 {
                        break;
                    }
                }
                FracOutput::MandelJulia{iters:iterations, max_iters:self.max_iter}
            }
            FractalType::Newton(roots) => {

                // The iteration for a Newton fractal (use the roots)
                let num_of_roots: usize = roots.len();

                // 'z' starts at 'z_in' and gets decremented by polynomial
                // over derivative
                let mut z: MyComplex<f32> = z_in;

                // The polynomial and derivative are complex numbers.
                let mut poly: MyComplex<f32>;
                let mut deriv: MyComplex<f32>;
                let mut poly_term: MyComplex<f32>;
                // 'partial' is used to sum up all of the product rule terms
                // of the derivative
                let mut partial: MyComplex<f32>;
                // Use Newton's method enough times to get z_in to converge to
                // a root
                for _ in 0..self.max_iter {
                    // Start poly at 1, the multiplicative identity
                    poly = MyComplex::new(1.0, 0.0);
                    // Start deriv at 0, the additive identity
                    deriv = MyComplex::new(0.0, 0.0);

                    // Loop over all of the roots in the polynomial
                    for j in 0..num_of_roots {
                        // Calculate the polynomial by multiplying each of
                        // the terms together
                        poly_term = z - roots[j];
                        poly *= poly_term;

                        partial = MyComplex::new(1.0, 0.0);
                        for k in 0..num_of_roots {
                            if k != j { partial *= z - roots[k]; }
                        }

                        // Calculate the derivative by summing together the
                        // product rule terms
                        deriv += partial;
                    }

                    z -= poly/deriv;
                }
                
                // Calculate which root the point is now closest to (assume
                // that the first root is the closest, just to get a starting
                // value to compare off of.
                let mut closest_root: usize = 0;
                let mut smallest_diff: f32 = (z - roots[0]).mag_sqr();
                let mut diff: f32;
                for j in 0..num_of_roots {
                    diff = (z - roots[j]).mag_sqr();
                    if diff < smallest_diff {
                        smallest_diff = diff;
                        closest_root = j;
                    }
                }
                // Return the closest root as a FracOutput
                // println!("{}, {}", closest_root, num_of_roots);
                FracOutput::Newton{closest:closest_root, roots:num_of_roots}
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
    // Check some known values on the Mandelbrot set, and make sure the divergence values make sense #[test]
    fn mandelbrot_test () {
        let fractal_to_test = Fractal {max_iter: 1_000_u32, kind: FractalType::Mandelbrot()};

        println!("testing...");
        let cmplx_in: [MyComplex::<f32>; 5] = [MyComplex::new(0.0, 0.0),
            MyComplex::new(1.0, 0.0), MyComplex::new(-0.3, -0.009),
            MyComplex::new(0.75, -0.1), MyComplex::new(0.26, 0.0)];
        let known_vals: [u32; 5] = [1000, 3, 1000, 3, 30];
        let mut mandel_vals: [u32; 5] = [0; 5];
        let mut i: usize = 0;
        while i < 5 {
            if let FracOutput::MandelJulia{iters:num, max_iters:_} = fractal_to_test.complex_to_frac_output(cmplx_in[i]) {
                mandel_vals[i] = num;
            }
            i += 1;
        }

        assert_eq!(known_vals, mandel_vals);
    }

    // Check some known values on a Julia set
    #[test]
    fn julia_test () {
        let fractal_to_test = Fractal {max_iter: 1_000_u32, kind: FractalType::Julia(MyComplex::new(0.2, -0.17))};

        let cmplx_in: [MyComplex::<f32>; 5] = [MyComplex::new(0.0, 0.0),
            MyComplex::new(1.0, 0.0), MyComplex::new(-0.3, -0.009),
            MyComplex::new(0.75, -0.1), MyComplex::new(0.26, 0.0)];
        let known_vals: [u32; 5] = [1000, 3, 1000, 1000, 1000];
        let mut julia_vals: [u32; 5] = [0; 5];
        let mut i: usize = 0;
        while i < 5 {
            if let FracOutput::MandelJulia{iters:num, max_iters:_} = fractal_to_test.complex_to_frac_output(cmplx_in[i]) {
                julia_vals[i] = num;
            }
            i += 1;
        }

        assert_eq!(known_vals, julia_vals);
    }

    // Check some known values on a Julia set
    #[test]
    fn newton_test () {
        let test_roots: Vec<MyComplex<f32>> = vec![MyComplex::new(1.0, 0.0),
            MyComplex::new(0.5, 0.5), MyComplex::new(-0.5, -0.5)];
        let fractal_to_test = Fractal {max_iter: 1_000_u32, kind: FractalType::Newton(test_roots)};

        let cmplx_in: [MyComplex::<f32>; 5] = [MyComplex::new(0.0, 0.0),
            MyComplex::new(1.0, 0.0), MyComplex::new(-0.3, -0.009),
            MyComplex::new(0.75, -0.1), MyComplex::new(0.26, 0.0)];
        let known_vals: [usize; 5] = [2, 0, 2, 0, 2];
        let mut newton_vals: [usize; 5] = [0; 5];
        let mut i: usize = 0;
        while i < 5 {
            if let FracOutput::Newton{closest:num, roots:_} = fractal_to_test.complex_to_frac_output(cmplx_in[i]) {
                newton_vals[i] = num;
            }
            i += 1;
        }

        assert_eq!(known_vals, newton_vals);
    }
}
