fn main() {
   
    let result = convert_fahrenheit_to_celsius(212);
    println!("The temperature is {result}");
    let nth_fib= nth_fibonacci_number(5);
    println!("The nth_fibonnaci {nth_fib} ");
}

fn convert_fahrenheit_to_celsius(temperature : i32)->i32{
    (temperature-32)*5/9
}

fn nth_fibonacci_number(n:u32) -> u32{
    if n<=0 {return 0;}
    if  n==1 {return n;}
    nth_fibonacci_number(n-1)+nth_fibonacci_number(n-2)
}
