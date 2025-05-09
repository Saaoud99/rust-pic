use std::io;

fn main() -> io::Result<()> {
    let mut count = 0;
    loop{
        count += 1;
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("gg");
        if input.trim() == "The letter e"{

            println!("Number of trials: {}", count);
            break;
        }
    }
    Ok(())
}