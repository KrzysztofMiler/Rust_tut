fn main() {
    let  prost = (50,30);
    

    println!(
        "prostokont ma pole {} pikseli kwadrat",
        pole(prost)
    );
}

fn pole(wymiar: (u32,u32)) -> u32 {
    wymiar.0*wymiar.1
}
