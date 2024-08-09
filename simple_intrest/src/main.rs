

use std::io;


fn main(){

    let p = getinput("please enter your principle Amount : eg rs 450.50");
    let t= getinput("please enter your time in years :eg 1.5");
    let r = getinput("please enter your rate in percent :eg 4.5");

    let si = (p * t * r)/100 as f64;

    println!("your simple intrest is {}",si);



}

fn getinput(prompt: &str)->f64{
   loop {
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin().read_line( &mut input).expect("failed to read input");

    match input.trim().parse::<f64>() {
        Ok(num) => {
            return num
        },
            Err(_) => {
                println!("Sorry, I couldn't parse your input. Please enter a valid number.");   
            },      
    }
   }

}