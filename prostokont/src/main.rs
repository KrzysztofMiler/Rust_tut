#[derive(Debug)]//konieczne by można było wyświetlić przez debug
struct Prost {
    wys : u32,
    szer :u32,
}



fn main() {
    let  prost1 = Prost {szer:30,wys:50};
    
    println!("prost 1 ma {:?}",prost1 );//:? powoduje że zamiast display 
    //mamy wyjście na debug, jeśli użyje :#? to dostaje z enterami zformatowane

    println!(
        "prostokont ma pole {} pikseli kwadrat",
        pole(&prost1)
    );
}

fn pole(prost: &Prost) -> u32 {
    prost.wys*prost.szer
    
}
