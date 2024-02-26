use std::io;

fn main() {
    println!("\n##########################################");
    println!("#          Tableaux et vecteurs          #");
    println!("##########################################\n\n");
    let tableau: [i32; 5] = [1, 2, 3, 4, 5];
    let mut v: Vec<i32> = Vec::new();
    let mut ordre = String::new();
    println!("Tableau : ");
    io::stdin().read_line(&mut ordre).expect("Echec de la lecture de l'entree utilisateur");
    while ordre != "exit\n" {
        if ordre == "sum\n" {sometab(&tableau);}
        if ordre == "print\n" {printab(&tableau);}
        if &ordre[..5] == "find " {findtab(&tableau, &ordre[5..ordre.len() - 1]);}

        ordre.clear();
        println!("Tableau : ");
        io::stdin().read_line(&mut ordre).expect("Echec de la lecture de l'entree utilisateur");
    }
    //---------------------------------------------------------
    ordre.clear();
    println!("Vecteur : ");
    io::stdin().read_line(&mut ordre).expect("Echec de la lecture de l'entree utilisateur");
    while ordre != "exit\n" {
        if ordre == "sum\n" {somevec(&v);}
        if ordre == "print\n" {printvec(&v);}
        if &ordre[..5] == "find " {findvec(&v, &ordre[5..ordre.len() - 1]);}
        if &ordre[..5] == "push " {pushvec(&mut v, &ordre[5..ordre.len() - 1]);}
        if ordre == "pop\n" {popvec(&mut v);}
        ordre.clear();
        println!("Vecteur : ");
        io::stdin().read_line(&mut ordre).expect("Echec de la lecture de l'entree utilisateur");
    }
    println!("FIN");
}
//-------------------------
fn sometab(tab: &[i32]) {
    let mut sum = 0;
    for &i in tab {
        sum += i;
    }
    println!("Total = {}", sum);
}

fn printab(tab: &[i32]) {
    let mut i: usize = 0;
    while i < tab.len() {
        println!("{} : [{}]", tab[i], i);
        i += 1;
    }
}

fn findtab(tab: &[i32], tofind: &str) -> usize {
    let mut i: usize = 0;
    let my_number: i32 = match tofind.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("chaine invalide");
            return 0;
        }
    };
    while i < tab.len() {
        if tab[i] == my_number {
            println!("Trouve : {}", i);
            return 1;
        };
        i += 1;
    }
    println!("l'element {} n'existe pas dans ce tableau", my_number);
    return 0
}
//-------------------------------------

fn somevec(tab: &Vec<i32>) {
    let mut sum = 0;
    for &i in tab {
        sum += i;
    }
    println!("Total = {}", sum);
}

fn printvec(tab: &Vec<i32>) {
    let mut i: usize = 0;
    while i < tab.len() {
        println!("{} : [{}]", tab[i], i);
        i += 1;
    }
}

fn findvec(tab: &Vec<i32>, tofind: &str) -> usize {
    let mut i: usize = 0;
    let my_number: i32 = match tofind.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("chaine invalide");
            return 0;
        }
    };
    while i < tab.len() {
        if tab[i] == my_number {
            println!("Trouve : {}", i);
            return 1;
        };
        i += 1;
    }
    println!("l'element {} n'existe pas dans ce tableau", my_number);
    return 0
}

fn pushvec(tab: &mut Vec<i32>, topush: &str) -> i32{
    let my_number: i32 = match topush.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("chaine invalide");
            return 0;
        }
    };
    tab.push(my_number);
    return 1;
}

fn popvec(tab: &mut Vec<i32>)
{
    tab.pop();
}
