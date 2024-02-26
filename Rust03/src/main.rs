fn main() {
    println!("\n##########################################");
    println!("#         Structures de controle         #");
    println!("##########################################\n");

    let mut i: u8 = 1;

    while i < 101 {
        if i % 2 == 0 {
            println!("\x1b[32m{} | Paire\x1b[0m", i);
        } else {
            println!("\x1b[31m{} | Impaire\x1b[0m", i);
        }
        i += 1;
    }
}
