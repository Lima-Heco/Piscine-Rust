use std::env;

fn main() {
    println!("#########################################");
    println!("#       Manipulation de variables       #");
    println!("#########################################\n");

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("\x1b[31m{}\x1b[0m", "Mauvaise utilisation");
        println!("./cargo run [num1] [operateur] [num2]\n\n\n");
        return;
    }
    //println!("{:?}", args);
    let number1: i32 = args[1].parse().expect("Erreur lors de la conversion en entier");
    let number2: i32 = args[3].parse().expect("Erreur lors de la conversion en entier");
    if args[2] == "+"{
        println!("{}", number1 + number2);
    }
    else if args[2] == "-"{
        println!("{}", number1 - number2);
    }
    else if args[2] == "*"{
        println!("{}", number1 * number2);
    }
    else if args[2] == "/"{
        println!("{}", number1 / number2);
    }
}
