pub fn delete_and_backspace(s: &mut String) {
   let mut stock = String::from("");
   let mut nbr : i32 = 0;

   for i in s.chars(){

       if nbr >= 1 && i != '-' && i != '+'{
           nbr -= 1;
           continue;
       }
       if i != '-' && i != '+' {
           stock.push(i);
       }

       if i == '-' {
           stock.pop();
           
       } else if i == '+' {
           nbr += 1;
       }
   }

   *s = stock;
}

pub fn do_operations(v: &mut [String]) {
    let mut nbr1 : f64;
    let mut nbr2 : f64;
    let mut nbr3 : f64;

   
   for i in v{
      if i.contains("+") || i.contains("-"){
        if i.contains("+"){
            let items: Vec<&str> = i.split("+").collect();
            
                nbr1 = items[0].parse().unwrap();
                nbr2 = items[1].parse().unwrap();
                nbr3 = nbr1+nbr2;

            *i= nbr3.to_string();
        }else{
            let items: Vec<&str> = i.split("-").collect();
            
            nbr1 = items[0].parse().unwrap();
            nbr2 = items[1].parse().unwrap();
            nbr3 = nbr1-nbr2;

            *i= nbr3.to_string();
        }
      }
   }
   
}