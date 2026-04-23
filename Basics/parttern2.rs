fn main() {
    
    for _ in 1..=5 {
        print!( "*");
    } 
    print!("\n");
    for i in (1..4).rev() {
        for _ in (1..=i).rev(){
            print!(" ");
        }
        print!("* \n");
    }
    for _ in 1..=5 {
        print!( "*");
    } 
   
}



// *****
//    *
//   *
//  *
// ***** 
