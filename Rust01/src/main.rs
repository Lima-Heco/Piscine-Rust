use std::io;

fn main() {
    let mut nom = String::new();
    println!("\n##########################################");
    println!("#         Premiers pas avec Rust         #");
    println!("##########################################\n\n");
    println!("Mais qui etes vous ?\n");
    io::stdin().read_line(&mut nom).expect("Echec de la lecture de l'entree utilisateur");
    println!("\nMais Bonjour {} !!\n", &nom[..nom.len() - 1]);
}
