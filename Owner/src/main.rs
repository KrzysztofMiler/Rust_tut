fn main() {

let mut s  = String::from("hello");
s.push_str(", wardo"); //push_str doddaje do stringa dlatego str mut
print!("{}",s);

let s1 = String::from("hello");
let s2 = s1;
//gdy robimy kopie to przekazujemy tylko pointer i unieważniamy
//nasz pkt który kopiowaliśmy czyli s1
println!("{}, world!", s2);

}
