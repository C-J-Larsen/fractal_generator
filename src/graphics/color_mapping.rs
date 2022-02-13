use super::super::fractals::FracVal;
use super::Color;

pub fn FracVal_to_Color(val: FracVal) -> Color {
    match val {
        FracVal::MandelJulia{iters:m_j_val, max_iters:max} => {
            let red: u8 = (m_j_val * 255 / max) as u8;
            Color { r:red, g:red, b:red }
        },
        FracVal::Newton{closest:n_val, roots:num_of_roots} => {
            match n_val {
                0 => Color { r: 255, g: 0, b: 0 },
                1 => Color { r: 0, g: 255, b: 0 },
                2 => Color { r: 0, g: 0, b: 255 },
                3 => Color { r: 0, g: 255, b: 255 },
                _ => Color { r: 30, g: 30, b: 30 }
            }
        }
    }
}
