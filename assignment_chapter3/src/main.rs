use std::thread::current;

fn main() {
   
    let result = convert_fahrenheit_to_celsius(212);
    println!("The temperature is {result}");
    let nth_fib= nth_fibonacci_number(5);
    println!("The nth_fibonnaci {nth_fib} ");
   
   let num = 21 ;

    println!(" is even {}",is_even(&num));

    fibbonacci_series();


}

fn convert_fahrenheit_to_celsius(temperature : i32)->i32{
    (temperature-32)*5/9
}

fn nth_fibonacci_number(n:u32) -> u32{
    if n<=0 {return 0;}
    if  n==1 {return n;}
    nth_fibonacci_number(n-1)+nth_fibonacci_number(n-2)
}


fn is_even(num:&u32) -> bool{
    num%2==0 
}

fn fibbonacci_series(){
    let mut  past =0;
    let mut  present =1;
    print!(" {past} {present} ");
    // let mut i =1;
    //  while i<=10 {
    //     let current = past + present;
    //     past = present;
    //     present = current;
    //     print!( " {current} ");
    //      i=i+1;
    //  }

     for  _ in 1..10 {
        let current = past + present;
        past = present;
        present = current;
        print!( " {current} ");
     }
}



fn length_of_string(input : &String) -> u32 {
   input.chars().count()
}

