use std::io; 

fn main (){
    let mut input = String::new();
    
    println!("enter a number");

    io::stdin()
    .read_line( &mut input)
    .expect("Something went wrong");

    let num : i64 = match input.trim().parse() {
        Ok ( num ) => num,
        Err(_) => {
            println!("something went wrong");
            return;
        } 
    };

    for i in 1..=10 {
        println!(" {} * {} = {} ", num, i , num * i)
    }
}