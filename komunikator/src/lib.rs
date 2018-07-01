#[cfg(test)]

pub mod network;

pub mod client;//to oznacza że rust szuka gdzie indziej to było zdeklarowane
//czyli szuka client.rs
    

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
