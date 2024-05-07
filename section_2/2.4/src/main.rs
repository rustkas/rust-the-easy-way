fn main() {
   
   let array = ["This", "is", "an", "array"];

   let array_slice = &array[1..4];

   println!("{array:?}");
   println!("{array_slice:?}");
}
