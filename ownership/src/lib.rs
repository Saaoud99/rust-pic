pub fn first_subword(mut s: String) -> String {
    let mut stock = String::new();
    for (index, i) in s.chars().enumerate(){
        if index != 0 && (i.is_uppercase() || i == '_'){
            break
        }else{
            stock.push(i);
        }
    }
    stock
}