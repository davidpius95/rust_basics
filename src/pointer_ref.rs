// REFERENCE POINTERS - POINT TO A RESOURCES IN MEMORY 


pub fn run() {
	 //primitive Array 
	 let arr1 = [1,2,3];
	 let arr2 = arr1;
	 
	 println!("Value: {:?}", (arr1,arr2));
	 // With non-premitive, If you assign another variable to a piece of data, the first variable will no longer hold that value. You will need to use a reference (&) to point to the resources
	// vector
	 let vec1 = vec![1,2,3];
	 let vec2 = &vec1;

	 println!("Value: {:?}", (&vec1,vec2));

}