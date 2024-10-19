pub fn test_match_int() {

  let value = 32;

  match value {

      10 => println!("The value is 10 "),
      _ => println!("The value is not 10 ")

  };

}


pub fn test_match_str() -> &str {
  let myName = "SridharVadla";

  match myName {
      "SridharVadla" => "Correct",
      _ => "Incorrect"

  }
}