use std ::io;

fn main (){
    
    let mut input = String::new() ;
    
    io::stdin()
    .read_line( &mut input)
    .expect("Fail to read input");
     
     
     let num :i64= match  input.trim().parse() {
        Ok( num ) => num,
        Err(_) => {
              println!("Please enter a valid number!");
            return;
        }  
    };
    
    for i in 1..=10 {
        println!( " {} * {} = {} ", num, i, i*num  )
    }
}