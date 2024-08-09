use std::io;

fn main(){
    let mut _default_input: i8 = 0;
    loop{
        opening_message();
        match  take_user_input() {
            Some(weight )=>{
                _default_input =  weight;
                break;
            }
            _=>{
                println!("sorry can't parse the number here try again");
                continue;
            }
        }
    }
    show_user_input( & _default_input);

  let realweight: f32 =  calculate_earth_weight(_default_input);
  show_result(realweight);

}


fn show_result(realweight:f32){


    println!();
    println!("🚀🌟 *** WEIGHT ON MARS *** 🌟🚀");
    println!("Your weight on Mars:  {:.2} kg", realweight);
    println!();
    
    println!("╭────────────────────────────────────────╮");
    println!("│                                        │");
    println!("│    🎉 You would weigh {:.2} kg on Mars! 🎉", realweight);
    println!("│                                        │");
    println!("│          🌌 Explore the Red Planet! 🌌    │");
    println!("│                                        │");
    println!("╰────────────────────────────────────────╯");
    println!();

}


fn calculate_earth_weight(input:i8)-> f32{
    let raw_result: f32 =  (input as f32 /9.81) * 3.77;
    raw_result
}


fn show_user_input(getter :& i8){
println!();
println!("🌟✨ *** SELECTION CONFIRMED *** ✨🌟");
println!("🛡️ You've chosen:  {}", getter);
println!();
println!("╭──────────────────────────╮");
println!("│                          │");
println!("│  🎯 Your selected input  │");
println!("│          is:  {}          │",getter);
println!("│                          │");
println!("│                          │");
println!("╰──────────────────────────╯");
println!();
println!("🚀 Ready to proceed with your choice! 🚀");

}

fn take_user_input() -> Option<i8>{
    let mut user_input = String::new();
    give_input_message();
    io::stdin().read_line(&mut user_input).expect("sorry can't read your input");

     match user_input.trim().parse::<i8>() {
        Ok(num)=>{
            Some(num)
        }
        Err(e)=>{
            println!("sorry there was an error while parsing");
            println!("the exact error {}",e);
            None
            
        }
        
    }
}


fn give_input_message(){
println!();
println!("🌍🚀 *** WEIGHT ENTRY *** 🚀🌍");
println!("📝 Please enter your weight on Earth:");
println!();
println!("╭──────────────────────────────╮");
println!("│                              │");
println!("│    📏 Your Earth weight is:  │");
println!("│                              │");
println!("╰──────────────────────────────╯");
println!();
println!("🌟 Ready to calculate your Martian weight! 🌟");
}



fn opening_message(){

    println!("🌌🚀 *** WELCOME TO THE MARS TO EARTH WEIGHT CALCULATOR *** 🚀🌌");
    println!();
    println!("🪐✨ Ready to find out how much you weigh on Mars compared to Earth? ✨🪐");
    println!("💫 Let's embark on this interplanetary journey and discover your Martian weight! 💫");
    println!();
    println!("╭──────────────────────────────────────────────╮");
    println!("│                                              │");
    println!("│ 🚀       Blast off into space!         🚀    │");
    println!("│                                              │");
    println!("│ 🌠     Explore your cosmic weight!     🌠    │");
    println!("│                                              │");
    println!("╰──────────────────────────────────────────────╯");
    println!();
    println!("🪐🚀 Prepare for lift-off and let’s calculate! 🚀🪐");

}