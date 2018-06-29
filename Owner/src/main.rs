fn main() {

let mut s  = String::from("hello");
s.push_str(", wardo"); //push_str doddaje do stringa dlatego str mut
print!("{}",s);

let s1 = String::from("hello");
let s2 = s1;
//gdy robimy kopie to przekazujemy tylko pointer i unieważniamy
//nasz pkt który kopiowaliśmy czyli s1 nazywa się to move
println!("{}, world!", s2);



let s3 = String::from("hello");//tutaj kopiujemy wszystko
let s4 = s3.clone();//łącznie z zawartością
// istnieje trait copy który robi to samo ale dla int,float,struct itd.

println!("s1 = {}, s2 = {}", s3, s4);

let g = String::from("waddap");//powwstaje g
takes_ownership(g);//wchodzi do funkcji
//już nie iestnieje
let x = 5; // powstaej x
make_copy(x);// wchodzi do fn. ale to kopja i32 wienc moszna
//używać x potem






}

fn takes_ownership(jakis_string: String){
    println!("{}",jakis_string );
}
fn make_copy(int:i32){
    println!("{}",int );



}