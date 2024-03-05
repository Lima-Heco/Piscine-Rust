fn safe_divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(x / y)
    }
}

fn find_element<T: PartialEq>(array: &[T], element: &T) -> Option<usize> {
    for (index, item) in array.iter().enumerate() {
        if item == element {
            return Some(index);
        }
    }
    None
}

fn main() {
    println!("\n############################################");
    println!("#           Gestion des erreures           #");
    println!("############################################\n\n");

    let result = safe_divide(10.0, 2.0);
    match result {
        Ok(value) => println!("Result of division: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    let array = vec![1, 2, 3, 4, 5];
    let element_to_find = &3;
    match find_element(&array, element_to_find) {
        Some(index) => println!("Element found at index: {}", index),
        None => println!("Element not found"),
    }
}
