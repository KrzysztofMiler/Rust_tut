use std::io;

fn main() {
    println!("Zgaduj liczbe!");
    println!("Zgaduj");

    let mut zgad = String::new();

    io::stdin().read_line(&mut zgad)
        .expect("nic nie wpisano");

    println!("Zgadłeś {}",zgad );
}
