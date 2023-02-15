    // -------------------------------------------
    // 			Rust Ownership 
    // 			- Each value in Rust has a variable that’s called its owner.
    // 			- There can be only one owner at a time.
    // 			- When the owner goes out of scope, the value will be dropped.
    // -------------------------------------------
    fn  stack_function(mut var:i32){
        var = 56;
        println!("var:{}",var);
    }
    fn heap_function(var:&mut Vec<i32>){
        var.push(50);
        println!("var: {:?}",var);
    }
    fn main()
    {
        let stack_num = 32;
        let mut heap_vec = vec![4,5,6];
    
        stack_function(stack_num);
        println!("The value inside the main of stack_num {}",stack_num);
    
        /*
        when  we pass a variable to a function which is stored on stack there is variable  of primitive type  such as integers
        so the function will  make the copy  of that and will use the copy and the move operation does not take place in this case. 
        it's mean the updation of the value inside the function does  not affect on the original value of the variables
        */
    
    
        heap_function(&mut heap_vec);
        print!("The value inside the main of heap_vec:{:?}",heap_vec);
    
    
        //How we concatenate two strings?
        let large_data1 = String::from("This is the first long String");
        let large_data2 = String::from("This is th second long string");
    
        let huge_data = vec![&large_data1,&large_data2];
        println!("The string in huge_data is {:?}",huge_data);
    
    }