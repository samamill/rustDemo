fn main() {
	let s = String::from("hello");
	let r1 = &s; // no problem
	let r2 = &s; // no problem
    println!("{}", s);
	println!("{}", r1);
	println!("{}", r2);
	
	println!("---------------");
	
	let mut t = String::from("hello");
    println!("{}", &mut t);
	{
		let u = &mut t;
		println!("{}", u);
	}
	
	t = String::from("world");
    println!("{}", &mut t);
	{
		let u = &mut t;
		println!("{}", u);
	}
}
