fn main() {
    let x =3;

    if x % 4 == 0{
    println!("div 4");
    }
    else if x % 3 == 0{
        println!("div 3", );
    }
    else {
        println!("div 1", );
    }
    
    let war = true;
    let num = if war{
        5
    }else{
        6
    };
    println!("wychdozi {}", num);
}
