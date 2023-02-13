/*in this section we learn how to make a function and code block */
fn main(){
    basic_fn();
    function_with_inputs("Attay", 80_000);
    let my_name = "AttayRasol";
    let my_salary = 90_000;
    
    
    function_with_inputs(my_name, my_salary);
    let answer = function_with_inputs_outputs(10,15);
    println!("The answer of multiplication is {}",answer);
 
 
    // first step to print the tupule
    let (multiplication,addition,subtraction) = function_with_inputs_multiple_outputs(10, 20);
    println!("Multiplication = {},Addition = {} , Subtraction = {}",multiplication,addition,subtraction);
 
    //Second step to print the tupule
    let result = function_with_inputs_multiple_outputs(3, 6);
    println!("Multiplication = {},Addition = {} , Subtraction = {}",result.0,result.1,result.2);
 
    let full_name:String = {
       let first_name = "Attay";
       let last_name = "Rasool";
       format!("{}  {} ",first_name,last_name)
 
    };
    println!("My full name is {} ", full_name);
 
 
    //How to take input from user?
 
    let mut n =  String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("Failed to read output.");
 
    let n:f64 = n.trim().parse().expect("invalid input");
    println!("{:?}",n);
 
 
 }
 
 
 fn basic_fn(){
    println!("this is the basic function with no outputs");
 }
 fn function_with_inputs(name: &str, salary: i32)
 {
    println!("my name is {} and my salary is Rs.{}", name, salary);
 }
 fn function_with_inputs_outputs(number1:i32,number2:i32) -> i32 {
    number1*number2
 }
 fn function_with_inputs_multiple_outputs(num1:i32,num2:i32) -> (i32,i32,i32){
    (num1*num2,num1+num2,num1-num2)
 }