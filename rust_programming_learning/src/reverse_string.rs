pub fn reverse(input: &str) -> String {

   let mut result = String::new();  
   let mut  i  =input.len();
   while i>=1 {

    if let Some(characters) = input.chars().nth(i-1){
      result.push(characters);
    }

    i-=1;

   }
   result
}
