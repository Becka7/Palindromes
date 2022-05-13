use std::io;
// use std::any::type_name_of_val;

fn main(){

    println!("Input a palindrome");

    let mut word = String::new();

    io::stdin()
    .read_line(&mut word)
    .expect("Oops!! could'nt read word");

    let d: String = word.chars().rev().collect();

    println!("Your reverse string is: {}",d);
    
    // match word == d {
    //     true => println!("It's a Palindrome !!!"),
    
    //     _=> println!("Oops ...it's not a palindrome"),
    // }

    // println!("the reverse is{}",d);
    // println!("the original word{}",word);
    // println!("{}", word.type());

    let myd: &str = &d;
    let myword: String =word.to_owned();
    
    println!("the reverse is{}",myd);
    println!("the original word{}",myword);
    println!("{}", myd == myword);
    


    // if  3 == 3{
    //     println!("{}", "abba".eq("abba"));
    //     println!("the reverse is{}",d);
    //     println!("the original word{}",word)
        
    // } else {
    //     println!("booooo");
    // }


}

