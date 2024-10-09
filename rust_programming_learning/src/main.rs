pub mod reverse_string;
pub mod giga_seconds;
use time::PrimitiveDateTime;
fn main() {
   let result = reverse_string::reverse("Sridhar");
   // print!("The reserve value  : {0} ",result);
   let start = PrimitiveDateTime::new(Date(2024-09-27), time!(10:00));
   let currentDate = giga_seconds::after(start);
}



