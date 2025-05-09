pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let y: f64 = c.into() ;
    (c, y.exp(), y.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let  stock = a.split(" ");
    let mut case = String::new();
    for i in stock{
        let num: f64 = i.parse().unwrap();
        let hey = num.exp();
        let  conv = hey.to_string();
        case.push_str(&conv);
        case.push_str(" ");
    }
    let res = case.trim();
    (a, (&res).to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut stock: Vec<f64> = Vec::new();

    for i in 0..b.len(){
        let y: f64 = b[i].into();
        let   goog = y.ln();
        stock.push(goog);
    }
    (b, stock)
}