#[derive(Debug)]
struct Prost {
    wys: u32,
    szer: u32,
}

impl Prost {
    fn pole(&self) -> u32 {
        self.wys * self.szer
    }
    fn zmiesci(&self, other: &Prost) -> bool {
        self.szer > other.szer && self.wys > other.wys
    }
}

fn main() {
    let prost1 = Prost { szer: 30, wys: 50 };
    let prost2 = Prost { szer: 20, wys: 40 };
    let prost3 = Prost { szer: 40, wys: 56 };

    println!("prost 1 ma {:?}", prost1);

    println!("prostokont ma pole {} pikseli kwadrat", prost1.pole()); //tak przekazywać do metod

    println!("czy prost 1 zmieści prost 2 ? {}", prost1.zmiesci(&prost2));
    println!("czy prost 1 zmieści prost 3 ? {}", prost1.zmiesci(&prost3));
}
