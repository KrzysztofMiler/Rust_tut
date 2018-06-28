extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Zgaduj liczbe!");

    let sec = rand::thread_rng().gen_range(1,101);
    println!("sikrit numba {}",sec );
loop{
    println!("Zgaduj");

    let mut zgad = String::new();

    io::stdin().read_line(&mut zgad)
        .expect("nic nie wpisano");

    let zgad:u32=zgad.trim().parse()
        .expect("liczba pls");

    println!("Zgadłeś {}",zgad );

    match zgad.cmp(&sec){
        Ordering::Less=> println!("za nisko", ),
        Ordering::Greater=> println!("za wysoko", ),
        Ordering::Equal=> {
            println!("gratz", );
            break;
        }
    }
}

}
