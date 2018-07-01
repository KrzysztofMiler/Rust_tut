#![allow(unused_variables)]
fn main() {
#[derive(Debug)]
enum UsState {
   Alabama,
   Alaska,
}

enum Coin {
   Penny,
   Nickel,
   Dime,
   Quarter(UsState),
}
let coin = Coin::Penny;
let mut count = 0;
if let Coin::Quarter(state) = coin {// taki match tylko dla mniejszej liczby zwrotów
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
}