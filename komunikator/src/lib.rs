#[cfg(test)]

pub mod network;

pub mod client;//to oznacza że rust szuka gdzie indziej to było zdeklarowane
//czyli szuka client.rs
    
#[cfg(test)]
mod tests {   
    use  super::client;

     #[test]
    fn it_works() {
       client::connect();//albo można zostawić tylko :: i wtedy rozpoczynamy od roota
    }
}
