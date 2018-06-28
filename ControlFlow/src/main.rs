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


    let mut i = 0;
    loop{
        i = i+1;        
        println!("zrobione razy{}",i );
        if i ==5{
            break
        }
    }

    let mut n =4;
    while   n !=0{
        println!("{}!", n);
        n = n-1;
    }

    let a = [10,20,30,40,50];

    for ele in a.iter(){
        println!("val is{}",ele );
    }

}
