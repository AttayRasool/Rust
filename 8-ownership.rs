                            // Rust Ownership



fn main()
{
    // integer is primitive type  thats why in integer we can move ownership
    //primitve type has fixed size and non empty
    //ownership srf primitive type may move ho skti hai non primitive type may nai move ho skti.
    
     let x  =  64.4;
     let y = x;
     println!("x:{} , y:{}",x,y);


    //but non primitive type  can be grow and can be empty
    //string is non primitve  and let see the code of ownership

     let s1 = String::from("attay");
    // this is the wrong way of moving ownership in string
    //let s2 = s1;

    //The right way is using & in  string.
    // & is using for reference
    //and in this case ownership is not changed
    //& it's borrow the value
    let s2 = &s1;


    println!("s1:{},s2:{}",s1,s2);

    //vectors are non primitive type
    //it can grow in size
    //and non empty
    //we use {:?} for printing
    /*
    1.we use &  in vector same as string
    2.we use vec.clone(); for  ownership
    */
    let vec_1 = vec![2,3,4,5,6];
    //let vec_2 = &vec_1;
    let vec_2 = vec_1.clone();

    println!("Vec 1:{:?} , Vec 2{:?}",vec_1,vec_2);




}