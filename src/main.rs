use std::{io};

fn main() {
    println!("please enter the first number");
    let mut first_value = String::new();

    io::stdin().read_line(&mut first_value).expect("you didn't enter a value");
    let first_value: u32 = first_value.trim().parse().expect("please enter a number");

    println!("please enter the second number");
    let mut second_value = String::new();

    io::stdin().read_line(&mut second_value).expect("you didn't enter another value");
    let second_value: u32 = second_value.trim().parse().expect("please enter a number");

    println!("what would you like to do with these two numbers?....");
    println!("Choose '+', if you want to add");
    println!("Choose '-', if you want to subtract");
    println!("Choose '*', if you want to multiply");
    println!("Choose '/', if you want to divide");
    let mut r_esult = String::new();

    io::stdin().read_line(&mut r_esult).expect("you havent chosen an option");
    

    let r_esult = r_esult.trim();

    
    if r_esult == "+" {
        println!("{:?}", first_value + &second_value);
    } else if r_esult == "-" {
        println!("{:?}", first_value - &second_value);
    } else if r_esult == "*"  {
        println!("{:?}", first_value * &second_value);
    } else if r_esult == "/" {
        println!("{:?}", first_value / &second_value);
    } else {
        println!("you haven't told me what you plan to do with both numbers");
    }

    println!("{:?}", r_esult);
}
