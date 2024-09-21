

fn main() {


    let mut str = String ::from("Hello ");
    str.push_str("World");

    println!("{str}");

   takes_ownership(&str);

    println!("After move : {str}");


    let x =5;

    copy_value(x);

    println!("After moving x into copy_value fn {x}");



    }

    fn copy_value(num1:i32){
        println!("{num1}");
    }

    fn takes_ownership(str : &String) -> (){
        println!("{str}");
        // return str;
    }
