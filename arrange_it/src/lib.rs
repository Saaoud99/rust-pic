pub fn arrange_phrase(phrase: &str) -> String {
    let  bra: Vec<&str> = phrase.split(" ").collect();
    let mut stock = vec![String::new(); bra.len()];

    for i in bra{
        for j in i.chars(){
            if j >= '1' && j <= '9'{
                let nbr = j.to_string().parse::<usize>().unwrap();
                let emp = i.replace(j, "");
                stock[nbr -1] = emp.to_string(); 
            }
        }
    }
    stock.join(" ")
}