mod maths;
//mod string_utils;

pub use crate::maths::math;
//pub use crate::string_utils::string_utils;

fn main() {
    println!("Addition: {}", math::add(5, 3));
    println!("Subtraction: {}", math::subtract(10, 7));
}