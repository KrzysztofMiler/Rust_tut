extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Zgaduj liczbe!");

    let sec = rand::thread_rng().gen_range(1,101);
    println!("sikrit numba {}",sec );

    println!("Zgaduj");

    let mut zgad = String::new();

    io::stdin().read_line(&mut zgad)
        .expect("nic nie wpisano");

    println!("Zgadłeś {}",zgad );
}
