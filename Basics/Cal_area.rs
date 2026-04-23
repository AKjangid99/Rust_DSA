use std::io;

fn main() {
    let mut input1 = String::new(); 
    let mut input2 = String::new(); 

    println!("Enter the length :");

    io::stdin()
    .read_line(&mut input1)
    .expect("Fail to read input");

    let len :i64= match  input1.trim().parse() {
        Ok( num ) => num,
        Err(_) => {
              println!("Please enter a valid number!");
            return;
        }  
    };

   println!("Enter the width :");

    io::stdin()
    .read_line( &mut input2 )
    .expect( " Fail to take input" );

    let wid : i64 = match input2.trim().parse(){
        Ok( num ) => num,
        Err(_) => {
              println!("Please enter a valid number!");
            return;
        } 
};

    let result = cal_area( len , wid );
    println!( "result {}" , result)

}

fn cal_area( l :i64, w:i64 ) -> i64{
    return l * w
}  

