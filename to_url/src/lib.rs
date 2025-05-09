pub fn to_url(s: &str) -> String {
    let mut stock = String::from("");

    for i in s.chars(){
        if i == ' '{
            stock.push_str("%20");
        }else{
            stock.push(i);
        }
    }
    return stock;
        
}