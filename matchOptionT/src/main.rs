
#![allow(unused_variables)]
fn main() {
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

println!("sześć to {:?}", six);
println!("null to {:?}",none );

let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),//jak wpiszemy _ to wtedy wszytkie opcje które nie były opisane wcześniej
    }//to tego miejsca są przyrównywane
}