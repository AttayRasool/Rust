fn main()
{
   /*in vector following function we can apply on vectors
   1.variable.len();

   //////we can check index by this function k  ye index is memory may hai k nai. ////////
   /////                                                                         ////////
   /////                 2.variable.get(index:);                                 ////////
   /////                                                                         ////////
   /////                                                                         ////////
   //////////////////////////////////////////////////////////////////////////////////////



   3.variable.push();
   4.variable.remove();

   agar koi number check krna ho k ye number is memory may hai k nai then hum ye function istemal karay gay


                                 5.varibale.contains(&10);




   */
   println!("Helllo world");
   //all the elements in vectors has same value datatypes
   let  mut num_vec = vec![4,5,6,78,9,10,12];
   //this statement for printing is error
//
//
   //
   //println!("{}",num_vec);
   //
   //
   //
   println!("{:?}",num_vec);
   println!("{:?}",num_vec[0]);
   println!("{:?}",num_vec[1]);
   println!("{:?}",num_vec[2]);
   println!("{:?}",num_vec[3]);
   println!("{:?}",num_vec[4]);


   //How we update the value of any index

   num_vec[5] = 25;
   println!("The value of 5th element is updated from 10 to {:?}",num_vec[5]); 
   println!("{:?}",num_vec);

   ///
   // for addning some new elements in vector, then we use push function
   num_vec.push(30);
   num_vec.push(40);
   println!("the new list is {:?}",num_vec);

   //for removal of any index we use

   num_vec.remove(5);
   println!("After removal, our new list is{:?}",num_vec);

   // for checking index, we use
   let check_index = num_vec.get(100);
   println!("the given index is {:?}",check_index);

   //for checking element we use
   println!("The value of 10 exist:{}", num_vec.contains(&10));
   println!("The value of 78 exist:{}", num_vec.contains(&78));

}