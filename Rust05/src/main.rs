struct Objet {
    shape: Shape,
    strshape: String,
    color: Color,
    strcolor: String
}

impl Objet {
    fn new(shape: Shape, color: Color) -> Objet {
        let sstrshape = match shape {
            Shape::Circle => "Circle",
            Shape::Square => "Square",
            Shape::Triangle => "Triangle",
            Shape::Rectangle => "Rectangle",
        };
        let sstrcolor = match color {
            Color::Red => "Red",
            Color::Green => "Green",
            Color::Blue => "Blue",
            Color::Yellow => "Yellow",
            Color::Black => "Black",
            Color::White => "White",
        };
        let strshape = String::from(sstrshape);
        let strcolor = String::from(sstrcolor);
        Objet {
            shape,
            strshape,
            color,
            strcolor
        }
    }
}

struct Person {
    name: String,
    age: u32,
    gender: char,
    is_student: bool,
    object: Objet
}

impl Person {
    fn new(name: String, age: u32, gender: char, is_student: bool, object: Objet) -> Person {
        Person { name, age, gender, is_student , object}
    }

    fn display(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Gender: {}", self.gender);
        println!("Is student: {}", self.is_student);
        println!("object: a {} {}", self.object.strcolor, self.object.strshape);
    }
}

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    White,
}

enum Shape {
    Circle,
    Square,
    Triangle,
    Rectangle,
}




fn main() {
    println!("\n##########################################");
    println!("#       Structures et enumerations       #");
    println!("##########################################\n\n");

    let loic = Person::new(String::from("Loic HEINRICH"), 22, 'M', true, Objet::new(Shape::Circle, Color::Blue));
    loic.display();

}
