fn main() {

    const THREE_HOURS_IN_SECS: u32  = 3*60*60;
    println!("Seconds in Threes hours : {THREE_HOURS_IN_SECS}");
    let mut x =5;
    println!("The value of x : {x}");
    x=6;
    println!("The value of x : {x}");

    {
        let  x =5;
        println!("The value of inner scope  x : {x}");
    }

    let x = 3;
    println!("The value of x after shawdowing is {x}");

}
