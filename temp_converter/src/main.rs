use std::io;
fn main() {
    loop {
        println!("welcome to my temprature converter");
        println!("type 1 for celcius to fahrenheit.");
        println!("type 2 for fahrenheit to celcius.");

        let mut userinput = String::new();

        io::stdin().read_line(&mut userinput).expect("please give valid number.");

         let  decision:i8 = match userinput.trim().parse::<i8>() {
            Ok(num)=> num,
            Err(_)=>{
                println!("please it must be a number");
                continue;
            }
        };

        if decision == 1{
            let userinput = takeinput("please enter celsius:  ");
            let result = calculate_celsius_to_fahrenheit(userinput);
            println!("{} celsius ",userinput);
            println!("=");
            println!("{} fahrenheit ",result)

        }else if decision == 2{
            let userinput = takeinput("please enter fahrenheit:  ");
            let result = calculate_fahrenheit_to_celsius(userinput);
            println!("{}   fahrenheit ",userinput);
            println!("=");
            println!("{} celsius ",result);


        }else{
            println!("sorry only two conversion are available try again.");
            continue;
        }   
    }
}


fn takeinput(prompt:&str)->f64{
    loop {
        println!("{}",prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("sotty can't read input try again");
        match input.trim().parse::<f64>() {
            Ok(num)=>{
                return  num;
            }
            Err(_)=>{
                println!("sorry can't parse try again");
            }
            
        }
    }

}



fn calculate_celsius_to_fahrenheit(celsius:f64)-> f64{
   let  fahrenheit = (9 as f64 / 5 as f64 *celsius as f64)+ 32 as f64;
    return  fahrenheit;
}

fn calculate_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    celsius
}


// kjfdlksjsd