fn main() {
    let szer = 30;
    let wys = 50;

    println!(
        "prostokont ma pole {} pikseli kwadrat",
        pole(szer, wys)
    );
}

fn pole(szer: u32, wys: u32) -> u32 {
    szer * wys
}
