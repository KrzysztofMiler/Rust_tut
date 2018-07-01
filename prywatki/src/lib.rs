mod zewnetrzna{
    pub fn sr_funkcja(){}
    fn sr_tajna_funkcja(){}
    mod srodek{
        pub fn wew_funkcja(){}
        fn tajna_funkcja(){}
    }
}

fn pruba(){
    zewnetrzna::sr_funkcja();//przejdzie
    zewnetrzna::sr_tajna_funkcja();//nie przejdzie bo nie pub
    zewnetrzna::srodek::wew_funkcja();//nie przejdą bo mdo srodek nie jest pub
    zewnetrzna::srodek::tajna_funkcja();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
