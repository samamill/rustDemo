fn main() {
	let s1 = String::from("hello");
	let s2 = s1;
	let s4 = new_function(s2);
	println!("{}, world!", s4);
}

fn new_function(s3: String) -> String {
	println!("{}, world!", s3);
	s3
}
