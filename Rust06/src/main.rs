mod maths;
mod print_message;
//mod string_utils;

pub use crate::maths::math;

fn main() {
    println!("\n############################################");
    println!("#           Fonctions et modules           #");
    println!("############################################\n\n");
    println!("Addition: {}", math::add(5, 3));
    println!("Subtraction: {}", math::subtract(10, 7));
    print_message::print_message::print_hello();
}
