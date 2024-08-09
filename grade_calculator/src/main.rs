use std::io;
fn main(){

    let social = takeinput("please enter your social marks: ");

    let math = takeinput("please enter your math marks: ");

    let science = takeinput("please enter your science marks: ");

    let economics = takeinput("please enter your economics marks: ");

    let history = takeinput("please enter your history marks: ");

    let total_marks:[f64;5]= [social,math,science,economics,history];

    let total_number = calculate_total_marks(total_marks);

    let total_percent =  calculate_percent(total_number);

    get_grades(total_percent);





}

fn get_grades(percent: f64){
    if percent > 100.00{
        println!("sorry you are out of grades")
    }else if  percent > 90.00{
        println!("your grade is : A+");  
    }else if percent > 80.00{
        println!("your grade is : A");  


    }else if percent > 70.00{
        println!("your grade is : B+");  


    }else if percent > 60.00{
        println!("your grade is : B");  


    }else if percent > 50.00{
        println!("your grade is : C+");  


    }else if percent > 40.00{
        println!("your grade is : C");  


    }else{
        println!("sorry you have no grades");
        println!("please try again next time");
    }





}

fn calculate_percent(marks:f64)->f64{
    let total_percent = marks/5.00;
    return total_percent;

}



fn calculate_total_marks(totalmarks:[f64;5])-> f64{
    let mut total:f64 = 0.00;
    for v in totalmarks  {
        total = total  + v ;
    }
    return  total;
}

fn takeinput(prompt: &str)-> f64{
    loop {
        let mut  input = String::new();
        println!("{}",prompt);
        io::stdin().read_line(&mut input).expect("something went wrong try again");
        match input.trim().parse::<f64>() {
            Ok(num)=>{
                if num <= 100.00{
                    return  num;
                }else{
                    println!("sorry the marks cannot be over 100");
                    continue;
                }
            }
            Err(_)=>{
                println!("sorry couldn't parse your data try again");
            }   
        }   
    }
}