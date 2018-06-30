#![allow(unused_variables)]
fn main() {
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let dom = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}


//enum Message { 
 //   Quit,
//    Move { x: i32, y: i32 },
//    Write(String),
//    ChangeColor(i32, i32, i32),
//} //Przykøad æe enumki mogá mie© w sobie wszystko nawet inne enumy
/*impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();*/
