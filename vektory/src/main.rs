fn main() {
    let v1: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3, 4, 5]; //gdy damy wektorowi war. początkowe to nie musimy pisać specyfikacji typu

    let mut g = Vec::new();

    g.push(5);
    g.push(6);
    g.push(7);
    g.push(8);

    let third: &i32 = &v[2]; //przez odwołanie do indeksu
    let third: Option<&i32> = v.get(2); //przez f. get gdzie dostajemy Option<&T>

    //różnica jest taka, gdy z 1 opcji próbuje się odwołać do ele. 100 to program panikuje i crashuje(gdy niechce by tak sie stało)
    //opcja 2 zwraca None

    for i in &v {
        println!("{}", i);
    }

    let mut de = vec![100, 32, 57];
    for i in &mut de {
        *i += 50; // musimy dodać * przed i aby uzyskać wartośc w i przed kompilacją
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    row.push(SpreadsheetCell::Int(5));
    row.push(SpreadsheetCell::Text(String::from("gib")));//itd.
}
