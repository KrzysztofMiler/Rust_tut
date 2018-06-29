fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);//dajemy z & więc przekazujemy referencje a nie ownership

    println!("The length of '{}' is {}.", s1, len);

     let mut s = String::from("hello");

    change(&mut s);

    let reference_to_nothing = no_dangle();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {// s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}