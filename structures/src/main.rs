use std::fs::read_to_string;
use chrono;

struct Employee {
    user_name:String,
    id:u32,
}

fn main() {
    println!("Hello, world!");

    print!("Current time is  {}",chrono::Local::now());
}



fn find_by_id(num:u32) -> Option<Employee>{

   let sridhar = Employee{
    user_name : String::from("SridharVadla"),
    id:1,
   };

   if num == 1 {
    return Some(sridhar);
   }
   None

}
