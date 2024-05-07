fn main() {
	#[allow(unused_assignments)]
	let mut string1: String = String::new();

	string1 = "String".to_string();

	let string2 = &string1;

	println!("{:?}", string1);
	
	println!("{:?}", string2);

}
