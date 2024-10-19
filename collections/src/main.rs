pub mod match_test;

fn main() {

    // let mut list_of_name:Vec<String>  = Vec::new();
    // list_of_name.push(String::from("SridharVadla"));

    // let mut list_of_nums = vec![1,2,3];
    // println!("Before modification : {:#?}",list_of_nums);

    // list_of_nums.push(4);
    // list_of_nums.push(5);
    // list_of_nums.push(6);

    // println!("After  modification : {:#?}",list_of_nums);


    // let  element_at_2 = & list_of_nums[2];
    // println!("Element at second index : {}",element_at_2);


    //fetching elements using get methods 
    // let element_at_2_using_get_method=list_of_nums.get(2);
    // println!("Element at second index : {:#?}",element_at_2_using_get_method);
    // match element_at_2_using_get_method {
    //     Some(e) => println!("element at index 2 :{0}",e),
    //     None => print!("provide valid index number")
    // }


    // fetch overflow index  from vector data type 
    // let value_at_100 =&list_of_nums[100];
    // let value_at_100_methodcall = list_of_name.get(100);

    // match value_at_100_methodcall {
    //     Some(T) => println!("{0}",T),
    //     None => println!("provide valid index!")
    // }

    // println!("overflow value  : {0}",value_at_100);





    // println!("length of list_of_names : {0}",list_of_name.len());

  
    match_test::test_match_int();

}
