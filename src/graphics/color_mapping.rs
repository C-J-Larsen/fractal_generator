use super::super::fractals::FracOutput;
use super::Color;

pub fn FracOutput_to_Color(val: FracOutput) -> Color {
    match val {
        FracOutput::MandelJulia{iters:m_j_val, max_iters:max} => {
            Color {
                // Each color will have 10 different levels
                r: ((max - m_j_val) as u8 % 10) * 26,
                g: ((max - m_j_val) as u8 % 7 + 5) * 15,
                b: ((max - m_j_val) as u8 % 4 + 5) * 10
            }
        },
        FracOutput::Newton{closest:n_val, roots:num_of_roots} => {
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
