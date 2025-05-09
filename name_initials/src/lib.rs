pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut nam: Vec<String> = Vec::new();
    for i in names {
        let mut stock = String::new();
        let  hh = i.split(" ");
        for word in hh{
        if let Some(first_char) = word.chars().next(){
            stock.push(first_char);
            stock.push('.');
            stock.push(' ');
        }
    }
        let  hey  = stock.trim();
        nam.push(hey.to_string());
    }
    nam
}