use std::io;

fn main() {

    let mut input = String::new();
    
    io::stdin().read_line( &mut input).unwrap();
    
    let num : i32 = input.trim().parse().unwrap();
    
    if num%2 == 0 {
        println!("Even")
    } else{
        println!("Odd")
    }

}